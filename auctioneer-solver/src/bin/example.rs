use alloy_primitives::{Address as alloy_Address, U256 as alloy_U256};
use alloy_sol_types::{sol, SolCall};
use auctioneer_solver::{
    generate_raw_tx, run, Response, SolverClient, SolverSolution, UserReq, ANVIL_PORT, ANVIL_URL,
    AUCTIONEER_ADDRESS, AXIOM_V2_ADDRESS, PINNED_BLOCK, SOLVER_1_KEY, SOLVER_2_KEY, UNI_V2_ROUTER,
    UNI_V3_ROUTER, USDC_ADDRESS, USER_ADDRESS, USER_KEY, WETH_ADDRESS,
};
use dotenv::dotenv;
use env_logger::Env;
use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, U256};
use ethers::utils::{parse_ether, Anvil};
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;
abigen!(AuctioneerChallenge, "./src/abi/auctioneer_challenge.json",);

abigen!(
    WETH,
    r#"[
        function deposit() public payable
        function withdraw(uint wad) public
        function totalSupply() public view returns (uint)
        function approve(address guy, uint wad) public returns (bool)
        function transfer(address dst, uint wad) public returns (bool)
        function transferFrom(address src, address dst, uint wad) public returns (bool)
    ]"#
);

abigen!(UniV3SwapRouter, "./src/abi/uniswap_v3_router_1.json",);

sol!(
    #[derive(Debug)]
    function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts);
    function getAmountsIn(uint amountOut, address[] memory path) public view returns (uint[] memory amounts);
    function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] calldata path, address to, uint256 deadline);
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    }

    // Gets the Eth mainnet connection
    dotenv().ok();
    let mainnet_http_url = env::var("HTTP_RPC").unwrap_or_else(|e| {
        log::error!("Error: {}", e);
        return e.to_string();
    });

    // Sets up anvil instance for testing
    let _anvil = Anvil::new()
        .chain_id(1u64)
        .port(ANVIL_PORT)
        .fork(mainnet_http_url.clone())
        .fork_block_number(PINNED_BLOCK)
        .spawn();
    let anvil_endpoint = format!("http://localhost:{}", ANVIL_PORT);

    let deployer = Provider::<Http>::try_from(anvil_endpoint.clone())
        .expect("could not instantiate HTTP Provider");

    let (_, receipt) = AuctioneerChallenge::deploy(
        deployer.into(),
        (
            AXIOM_V2_ADDRESS.parse::<Address>()?,
            1u64,
            AUCTIONEER_ADDRESS.parse::<Address>()?,
        ),
    )?
    .send_with_receipt()
    .await?;
    let auctioneer_contract_address = receipt.contract_address.unwrap();

    // Sets up the server
    let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let _rpc_endpoint = format!("http://{}", rpc_addr);
    let _ = tokio::spawn(async move {
        run(rpc_addr, ANVIL_URL.to_string()).await?;
        Ok::<(), anyhow::Error>(())
    });
    log::info!("Auctioneer server running on: {}", rpc_addr.to_string());

    // Populates the initial user and UserReq
    let user_req_id = send_mock_user_req(rpc_addr.to_string(), ANVIL_URL.to_string()).await?;
    log::info!("Sending UserReq: {}", user_req_id);

    // Sets up the first solver
    log::info!("Starting Solver 1...");
    let solver_1 = SolverClient::new(&anvil_endpoint, SOLVER_1_KEY);
    log::info!("Solver 1 address: {}", solver_1.provider.signer().address());
    log::info!("Starting Solver 2...");
    let solver_2 = SolverClient::new(&anvil_endpoint, SOLVER_2_KEY);
    log::info!("Solver 2 address: {}", solver_2.provider.signer().address());

    // Gets Weth for both solvers and approve to the UniV2 and UniV3 Routers
    let weth = WETH::new(
        WETH_ADDRESS.parse::<Address>().unwrap(),
        solver_1.provider.clone(),
    );
    let _ = weth
        .approve(UNI_V2_ROUTER.parse::<Address>().unwrap(), U256::MAX)
        .send()
        .await?
        .await?;
    let _ = weth
        .deposit()
        .value(parse_ether(11).unwrap())
        .send()
        .await?
        .await?;
    let weth = WETH::new(
        WETH_ADDRESS.parse::<Address>().unwrap(),
        solver_2.provider.clone(),
    );
    let _ = weth
        .approve(UNI_V3_ROUTER.parse::<Address>().unwrap(), U256::MAX)
        .send()
        .await?
        .await?;
    let _ = weth
        .deposit()
        .value(parse_ether(11).unwrap())
        .send()
        .await?
        .await?;

    // Craft Solver 1's solution using Univwapv2
    let path = vec![
        // WETH address
        alloy_Address::parse_checksummed(WETH_ADDRESS, None).unwrap(),
        // USDt address
        alloy_Address::parse_checksummed(USDC_ADDRESS, None).unwrap(),
    ];

    // Generate swap calldata
    let v2_swap = swapExactTokensForTokensCall {
        amountIn: alloy_U256::from(10000000000u64),
        amountOutMin: alloy_U256::from(1u64),
        path,
        to: alloy_Address::parse_checksummed(USER_ADDRESS, None).unwrap(),
        deadline: alloy_U256::from(10000000000u64),
    };
    let call_data = v2_swap.encode();

    // Solution using Uniswapv2 pool
    let solver_1_addr = solver_1.provider.signer().address();
    let solver_1_solution = generate_raw_tx(
        solver_1.provider.clone(),
        solver_1_addr,
        UNI_V2_ROUTER.parse::<Address>().unwrap(),
        None,
        Some(call_data.into()),
    )
    .await?;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("Http://{}", rpc_addr.to_string()))
        .header("Content-Type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "auction_getReqFromId",
            "params": vec![json!(USER_ADDRESS), json!(user_req_id)],
            "id": "1"
        }))
        .send()
        .await?;
    let str_response = response.text().await?;
    let user_req: anyhow::Result<Response<UserReq>> =
        serde_json::from_str(&str_response).map_err(anyhow::Error::from);
    let user_req = user_req.unwrap().result;

    // Sends the solutions
    let params = vec![
        json!(solver_1_addr),
        json!(SolverSolution::new(vec![solver_1_solution.clone()])),
        json!(USER_ADDRESS),
        json!(user_req),
    ];
    log::info!("Sending solver solution: {:?}", params[1]);

    let response = client
        .post(format!("Http://{}", rpc_addr.to_string()))
        .header("Content-Type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "auction_sendSolutions",
            "params": params,
            "id": "1"
        }))
        .send()
        .await?;
    let str_response = response.text().await?;
    let user_req: anyhow::Result<Response<UserReq>> =
        serde_json::from_str(&str_response).map_err(anyhow::Error::from);
    let user_req = user_req.unwrap().result;

    let params = vec![
        json!(solver_1_addr),
        json!(SolverSolution::new(vec![solver_1_solution])),
        json!(USER_ADDRESS),
        json!(user_req),
    ];
    log::info!("Sending solver 2 solution: {:?}", params[1]);

    // Publish the winner
    let params = vec![
        json!(user_req),
        json!(auctioneer_contract_address),
        json!(ANVIL_URL.to_string()),
    ];

    let response = client
        .post(format!("Http://{}", rpc_addr.to_string()))
        .header("Content-Type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "auction_resolveSolutions",
            "params": params,
            "id": "1"
        }))
        .send()
        .await?;
    let str_response = response.text().await?;

    let _: anyhow::Result<Response<bool>> =
        serde_json::from_str(&str_response).map_err(anyhow::Error::from);
    Ok(())
}

/// Helper method to send a UserReq to the auctioneer
async fn send_mock_user_req(
    rpc_addr: impl Into<String>,
    anvil_url: impl Into<String>,
) -> anyhow::Result<u64> {
    let client = reqwest::Client::new();

    let user_client = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(anvil_url.into()).expect("could not instantiate HTTP Provider"),
        USER_KEY.parse::<LocalWallet>().unwrap(),
    ));

    let raw_tx = generate_raw_tx(
        user_client.clone(),
        USER_ADDRESS.parse::<Address>().unwrap(),
        AUCTIONEER_ADDRESS.parse::<Address>().unwrap(),
        Some(10),
        None,
    )
    .await?;

    let params = vec![
        json!(USER_ADDRESS),
        json!(WETH_ADDRESS),
        json!(10000000000u64),
        json!(USDC_ADDRESS),
        json!(raw_tx),
    ];

    let response = client
        .post(format!("Http://{}", rpc_addr.into()))
        .header("Content-Type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "auction_sendUserReq",
            "params": params,
            "id": "1"
        }))
        .send()
        .await?;

    let str_response = response.text().await?;
    let parsed_response: anyhow::Result<Response<UserReq>> =
        serde_json::from_str(&str_response).map_err(anyhow::Error::from);
    match parsed_response {
        Ok(res) => Ok(res.result.id),
        Err(_) => Err(anyhow::anyhow!("Failed to parse response")),
    }
}

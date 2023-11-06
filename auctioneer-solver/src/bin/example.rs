use auctioneer_solver::{
    generate_raw_tx, run, Response, SolverClient, UserReq, ANVIL_PORT, ANVIL_URL,
    AUCTIONEER_ADDRESS, PINNED_BLOCK, SOLVER_1_KEY, SOLVER_2_KEY, USDC_ADDRESS, USER_ADDRESS,
    USER_KEY, WETH_ADDRESS,
};
use dotenv::dotenv;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use ethers::utils::Anvil;
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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

    // Sets up the server
    let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let rpc_endpoint = format!("http://{}", rpc_addr);
    let _ = tokio::spawn(async move {
        run(rpc_addr, ANVIL_URL.to_string()).await?;
        Ok::<(), anyhow::Error>(())
    });

    // Populates the initial user and UserReq
    let user_req_id = send_mock_user_req(rpc_addr.to_string(), ANVIL_URL.to_string()).await?;
    println!("UserReq id: {}", user_req_id);

    // Sets up the first solver
    let solver_1 = SolverClient::new(&rpc_endpoint, SOLVER_1_KEY);
    let solver_2 = SolverClient::new(&rpc_endpoint, SOLVER_2_KEY);

    let user_req = solver_1
        .get_req_from_id(USER_ADDRESS.parse::<Address>().unwrap(), user_req_id)
        .await?;
    println!("usre_req: {:?}", user_req);

    // solver 1 solution using Uniswapv2 pool
    //let solver_1_solution =
    let solver_1_addr = solver_1.provider.signer().address();
    solver_1
        .send_solutions(
            solver_1_addr,
            user_req.solvers[&solver_1_addr].clone(),
            USER_ADDRESS.parse::<Address>().unwrap(),
            user_req.clone(),
        )
        .await?;

    // Todo:
    // -solver 1 submits solution using univ2
    // -solver 2 submits solution using univ3
    // -auctioneer compare who can return more
    // -update the user req and submit on-chain

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
        user_client,
        USER_ADDRESS.parse::<Address>().unwrap(),
        AUCTIONEER_ADDRESS.parse::<Address>().unwrap(),
        10,
        None,
    )
    .await?;

    let params = vec![
        json!(USER_ADDRESS),
        json!(WETH_ADDRESS),
        json!(10u64),
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

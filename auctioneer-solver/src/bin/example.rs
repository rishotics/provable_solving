use auctioneer_solver::{
    run, Response, SolverClient, UserReq, ANVIL_PORT, ANVIL_URL, AUCTIONEER_ADDRESS, PINNED_BLOCK,
    USDC_ADDRESS, USER_ADDRESS, USER_KEY, WETH_ADDRESS,
};
use dotenv::dotenv;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{transaction::eip2718::TypedTransaction, Address, TransactionRequest};
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
        .port(ANVIL_PORT)
        .fork(mainnet_http_url.clone())
        .fork_block_number(PINNED_BLOCK)
        .spawn();

    // Sets up the server
    let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let _ = tokio::spawn(async move {
        run(rpc_addr, ANVIL_URL.to_string()).await?;
        Ok::<(), anyhow::Error>(())
    });

    // Populates the initial user and UserReq
    setup(rpc_addr.to_string(), ANVIL_URL.to_string()).await?;
    let rpc_endpoint = format!("http://{}", rpc_addr);

    // Sets up the first solver
    let solver_1 = SolverClient::new(&rpc_endpoint);
    let solver_2 = SolverClient::new(&rpc_endpoint);

    // Todo:
    // -solver 1 submits solution using univ2
    // -solver 2 submits solution using univ3
    // -auctioneer compare who can return more
    // -update the user req and submit on-chain

    Ok(())
}

/// Helper method to populsate the UserReq with the tokens
async fn setup(rpc_addr: impl Into<String>, anvil_addr: impl Into<String>) -> anyhow::Result<()> {
    // Populate the user request
    let client = reqwest::Client::new();

    let user_client = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(anvil_addr.into()).expect("could not instantiate HTTP Provider"),
        USER_KEY.parse::<LocalWallet>().unwrap(),
    ));

    let nonce = user_client
        .get_transaction_count(USER_ADDRESS, None)
        .await?;

    let tx = TypedTransaction::Legacy(
        TransactionRequest::new()
            .to(AUCTIONEER_ADDRESS)
            .value(1000u64)
            .from(USER_ADDRESS.parse::<Address>().unwrap())
            .gas(100000000u64)
            .nonce(nonce),
    );

    let signed_tx = user_client.signer().sign_transaction(&tx).await?;
    let raw_tx = tx.rlp_signed(&signed_tx);

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
        Ok(res) => res,
        Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
    };

    Ok(())
}

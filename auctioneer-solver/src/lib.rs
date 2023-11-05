mod auctioneer;
mod error;
mod solver;
mod types;

use crate::auctioneer::{AuctionApiServer, AuctioneerApiImpl};
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub use self::error::Error;
pub use self::solver::SolverClient;
pub use self::types::{Response, UserReq};

pub const USER_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
pub const PINNED_BLOCK: u64 = 18490936u64;
pub const USER_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
pub const AUCTIONEER_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
pub const ANVIL_URL: &str = "http://localhost:8545";
pub const ANVIL_PORT: u16 = 8545;

pub async fn run(addr: SocketAddr, rpc_url: String) -> anyhow::Result<()> {
    let cors_middleware = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let service = ServiceBuilder::new().option_layer(Some(cors_middleware));

    let server = ServerBuilder::new()
        .set_middleware(service)
        .build(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address: {} -> {}", &addr, e))?;

    let _server_handler = server.start(AuctioneerApiImpl::new(rpc_url).into_rpc());
    log::info!("Server started on: {}", addr);

    let ctrl_c = tokio::signal::ctrl_c();

    let mut stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let _sig = stream.recv();

    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C received, shutting down");
        }
        else => {
            println!("Server stopped unexpectedly");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::types::{Response, SolverRequestResponse, SolverSolution, UserReq};

    use super::*;
    use serde_json::{json, Value};

    use dotenv::dotenv;
    use ethers::middleware::SignerMiddleware;
    use ethers::providers::{Http, Middleware, Provider};
    use ethers::signers::{LocalWallet, Signer};
    use ethers::types::{transaction::eip2718::TypedTransaction, Address, TransactionRequest};
    use ethers::utils::Anvil;
    use std::env;
    use std::sync::Arc;

    async fn setup() -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        run(addr, "https://eth.llamarpc.com".to_string()).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_server_status() -> anyhow::Result<()> {
        let _ = tokio::spawn(async move {
            let _ = setup().await;
        });

        // wait for the server to setup
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let client = reqwest::Client::new();
        let response = client
            .post(format!("Http://{}", rpc_addr.to_string()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "auction_getStatus",
                "params": Vec::<Value>::new(),
                "id": "1"
            }))
            .send()
            .await?;

        let str_response = response.text().await?;
        let parsed_response: anyhow::Result<Response<bool>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result, true);
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_request() -> anyhow::Result<()> {
        let _ = tokio::spawn(async move {
            let _ = setup().await;
        });

        // wait for the server to setup
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let client = reqwest::Client::new();

        let addr = "0x2e895C036c6DFb475b514B7B8E7eCC278E03dF47"
            .parse::<Address>()
            .unwrap();
        let user_reqs: Vec<UserReq> = vec![UserReq::default()];

        let response = client
            .post(format!("Http://{}", rpc_addr.to_string()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "auction_populateUser",
                "params": vec![json!(addr), json!(user_reqs)],
                "id": "1"
            }))
            .send()
            .await?;

        let str_response = response.text().await?;
        let parsed_response: anyhow::Result<Response<bool>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result, true);
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }

        let response = client
            .post(format!("Http://{}", rpc_addr.to_string()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "auction_getReq",
                "params": vec![json!(addr)],
                "id": "1"
            }))
            .send()
            .await?;

        let str_response = response.text().await?;
        let parsed_response: anyhow::Result<Response<SolverRequestResponse>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result.user_reqs.len(), 1);
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_send_solution() -> anyhow::Result<()> {
        let _ = tokio::spawn(async move {
            let _ = setup().await;
        });

        // wait for the server to setup
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let client = reqwest::Client::new();

        let addr = "0x2e895C036c6DFb475b514B7B8E7eCC278E03dF47"
            .parse::<Address>()
            .unwrap();
        let user_reqs: Vec<UserReq> = vec![UserReq::default()];

        let response = client
            .post(format!("Http://{}", rpc_addr.to_string()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "auction_populateUser",
                "params": vec![json!(addr), json!(user_reqs)],
                "id": "1"
            }))
            .send()
            .await?;

        let str_response = response.text().await?;
        let parsed_response: anyhow::Result<Response<bool>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result, true);
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }

        let solver_solutions = SolverSolution::default();
        let solver_addr = Address::default();
        let user_req = UserReq::default();

        let params = vec![
            json!(solver_addr),
            json!(solver_solutions),
            json!(addr),
            json!(user_req),
        ];

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
        let parsed_response: anyhow::Result<Response<UserReq>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result.solved, false)
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_send_user_req() -> anyhow::Result<()> {
        let _ = tokio::spawn(async move {
            let _ = setup().await;
        });

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

        // wait for the server to setup
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let client = reqwest::Client::new();

        let user_client = Arc::new(SignerMiddleware::new(
            Provider::<Http>::try_from(ANVIL_URL.to_string())
                .expect("could not instantiate HTTP Provider"),
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

        let response = client
            .post(format!("Http://{}", rpc_addr.to_string()))
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "auction_sendUserReq",
                "params": vec![json!(USER_ADDRESS), json!(WETH_ADDRESS), json!(10u64), json!(USDC_ADDRESS),json!(raw_tx)],
                "id": "1"
            }))
            .send()
            .await?;

        let str_response = response.text().await?;
        let parsed_response: anyhow::Result<Response<UserReq>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        match parsed_response {
            Ok(res) => {
                assert_eq!(res.result.solved, false)
            }
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }
        Ok(())
    }
}

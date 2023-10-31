mod auctioneer;
mod error;

use crate::auctioneer::{AuctionApiServer, AuctioneerApiImpl};
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

use self::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(addr).await?;

    Ok(())
}

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
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

    let _server_handler = server.start(AuctioneerApiImpl::new().into_rpc());
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

    use crate::auctioneer::Response;

    use super::*;
    use serde_json::{json, Value};

    async fn setup() -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        run(addr).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_server_request() -> anyhow::Result<()> {
        let _ = tokio::spawn(async move {
            let _ = setup().await;
        });

        // Brute force way to wait for the server to setup
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
}
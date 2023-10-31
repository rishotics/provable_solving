pub mod auctioneer;

use crate::auctioneer::{AuctionApiServer, AuctioneerApiImpl};
use jsonrpsee::server::ServerBuilder;
use pin_utils::pin_mut;
use std::net::SocketAddr;
use std::panic;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let _ = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(128 * 1024 * 1024)
                .build()?;

            let _task = async move {
                let server = ServerBuilder::new()
                    .http_only()
                    .build(addr)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to bind to {}", e));
                match server {
                    Ok(server) => {
                        let _server_handler = server.start(AuctioneerApiImpl::new().into_rpc());
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            };

            let _ = rt.block_on(async {
                let ctrl_c = tokio::signal::ctrl_c();
                let mut stream =
                    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
                let _sig = stream.recv();
                pin_mut!(_task, _sig, ctrl_c);
                tokio::select! {
                    _ = ctrl_c => {
                        println!("Ctrl+C received, shutting down");
                    }
                    else => {
                        println!("Server stopped unexpectedly");
                    }
                }

                Ok::<(), anyhow::Error>(())
            });

            Ok::<(), anyhow::Error>(())
        })?
        .join()
        .unwrap_or_else(|e| panic::resume_unwind(e));

    Ok(())
}

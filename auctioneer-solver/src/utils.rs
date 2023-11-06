use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::signers::{Signer, Wallet};
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use ethers::types::{Bytes, U256};
use ethers::utils::parse_ether;
use std::sync::Arc;

/// Generates a raw transaction from the given addresses and value
/// Called from both users and solvers
pub async fn generate_raw_tx(
    provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    from: Address,
    to: Address,
    value: Option<u64>,
    data: Option<Bytes>,
) -> anyhow::Result<Bytes> {
    let nonce = provider.get_transaction_count(from, None).await?;

    let (max_fee_per_gas, max_priority_fee) = provider.estimate_eip1559_fees(None).await?;

    let mut tx = TypedTransaction::Eip1559(
        ethers::types::Eip1559TransactionRequest::new()
            .from(from)
            .to(to)
            .chain_id(1)
            .nonce(nonce)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee),
    );

    tx.set_gas(U256::from(1000000u64));

    if let Some(value) = value {
        tx.set_value(parse_ether(value).unwrap());
    }

    if let Some(data) = data {
        tx.set_data(data);
    };

    let signed_tx = provider.signer().sign_transaction(&tx).await?;
    let raw_tx = tx.rlp_signed(&signed_tx);
    Ok(raw_tx)
}

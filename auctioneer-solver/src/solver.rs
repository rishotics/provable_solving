use crate::types::{
    AllUsersRequestResponse, Response, SolverRequestResponse, SolverSolution, UserReq,
};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::Address;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug)]
pub struct SolverClient {
    rpc_endpoint: String,
    client: Client,
    pub provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

#[allow(dead_code)]
impl SolverClient {
    pub fn new(rpc_endpoint: &str, solver_key: &str) -> Self {
        Self {
            rpc_endpoint: rpc_endpoint.to_string(),
            client: Client::new(),
            provider: Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(rpc_endpoint)
                    .expect("could not instantiate HTTP Provider"),
                // Default to Anvil's testing wallet
                solver_key
                    .parse::<LocalWallet>()
                    // .unwrap() for pure convinience :)
                    .unwrap(),
            )),
        }
    }

    pub async fn get_req(&self, user_addr: Address) -> anyhow::Result<SolverRequestResponse> {
        let params = json!([user_addr]);
        self.call_rpc("getReq", params).await
    }

    pub async fn get_all_reqs(&self) -> anyhow::Result<AllUsersRequestResponse> {
        self.call_rpc("getAllReq", json!([])).await
    }

    pub async fn get_status(&self) -> anyhow::Result<bool> {
        self.call_rpc("getStatus", json!([])).await
    }

    pub async fn send_solutions(
        &self,
        solver_addr: Address,
        solutions: SolverSolution,
        user_addr: Address,
        user_req: UserReq,
    ) -> anyhow::Result<UserReq> {
        let params = json!([solver_addr, solutions, user_addr, user_req]);
        self.call_rpc("sendSolutions", params).await
    }

    pub async fn get_req_from_id(&self, user_addr: Address, id: u64) -> anyhow::Result<UserReq> {
        let params = json!([user_addr, id]);
        self.call_rpc("getReqFromId", params).await
    }

    async fn call_rpc<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<T> {
        // Constructing the request payload
        let payload = json!({
            "jsonrpc": "2.0",
            "method": format!("auction_{}", method),
            "params": params,
            "id": "1"
        });

        // Sending the request to the server
        let response = self
            .client
            .post(&self.rpc_endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        // Converting the response to text
        let str_response = response.text().await?;

        // Parsing the text response
        let parsed_response: anyhow::Result<Response<T>> =
            serde_json::from_str(&str_response).map_err(anyhow::Error::from);

        // Returning the result from the parsed response
        match parsed_response {
            Ok(res) => return Ok(res.result),
            Err(_) => Err(anyhow::anyhow!("Failed to parse response"))?,
        }
    }
}

use crate::auctioneer::{
    AllUsersRequestResponse, Response, SolverRequestResponse, SolverSolution, UserReq,
};
use ethers::types::Address;
use reqwest::Client;
use serde_json::json;

pub struct SolverClient {
    rpc_endpoint: String,
    client: Client,
}

#[allow(dead_code)]
impl SolverClient {
    pub fn new(rpc_endpoint: &str) -> Self {
        Self {
            rpc_endpoint: rpc_endpoint.to_string(),
            client: Client::new(),
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

    pub async fn populate_user(
        &self,
        user_addr: Address,
        reqs: Vec<UserReq>,
    ) -> anyhow::Result<bool> {
        let params = json!([user_addr, reqs]);
        self.call_rpc("populateUser", params).await
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

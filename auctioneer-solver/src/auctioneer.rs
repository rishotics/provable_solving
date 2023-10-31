use crate::Error;
use async_trait::async_trait;
use ethers::types::{transaction::eip1559::Eip1559TransactionRequest, Address};
use hashbrown::HashMap;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<R> {
    jsonrpc: String,
    pub result: R,
    id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolverSolution {
    solutions: Vec<Eip1559TransactionRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolverSolutionResponse {
    success: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolverRequestResponse {
    user_reqs: Vec<UserReq>,
}

#[rpc(server, namespace = "auction")]
trait AuctionApi {
    #[method(name = "getReq")]
    async fn get_req(&self, user_addr: Address) -> RpcResult<SolverRequestResponse>;

    #[method(name = "getStatus")]
    fn get_status(&self) -> RpcResult<bool>;

    #[method(name = "sendSolutions")]
    async fn send_solutions(
        &self,
        solution_req: Request<SolverSolution>,
    ) -> RpcResult<SolverSolutionResponse>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReq {
    hold_token: Address,
    hold_amt: u64,
    want_token: Address,
}

#[derive(Debug, Clone, Default)]
pub struct AuctioneerApiImpl {
    pub user_reqs: HashMap<Address, Vec<UserReq>>,
}

impl AuctioneerApiImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl AuctionApiServer for AuctioneerApiImpl {
    async fn get_req(&self, user_addr: Address) -> RpcResult<SolverRequestResponse> {
        match self.user_reqs.get(&user_addr) {
            Some(user_reqs) => {
                let response = SolverRequestResponse {
                    user_reqs: user_reqs.clone(),
                };
                Ok(response)
            }
            None => Err(Error::UserNotFound("User not found".to_string()).into()),
        }
    }

    async fn send_solutions(
        &self,
        _solution_req: Request<SolverSolution>,
    ) -> RpcResult<SolverSolutionResponse> {
        todo!()
    }

    fn get_status(&self) -> RpcResult<bool> {
        Ok(true)
    }
}

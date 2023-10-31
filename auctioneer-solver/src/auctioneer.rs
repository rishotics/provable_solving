use async_trait::async_trait;
use ethers::types::{transaction::eip1559::Eip1559TransactionRequest, Address};
use hashbrown::HashMap;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<R> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: R,
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
    success: bool,
}

#[rpc(server, namespace = "auction")]
trait AuctionApi {
    #[method(name = "getReq")]
    async fn get_req(&self) -> RpcResult<Response<SolverRequestResponse>>;

    #[method(name = "sendSolutions")]
    async fn send_solutions(
        &self,
        solution_req: Request<SolverSolution>,
    ) -> RpcResult<Response<SolverSolutionResponse>>;
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
    async fn get_req(&self) -> RpcResult<Response<SolverRequestResponse>> {
        todo!()
    }

    async fn send_solutions(
        &self,
        _solution_req: Request<SolverSolution>,
    ) -> RpcResult<Response<SolverSolutionResponse>> {
        todo!()
    }
}

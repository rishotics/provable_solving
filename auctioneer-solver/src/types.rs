use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<R> {
    pub jsonrpc: String,
    pub result: R,
    pub id: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct SolverSolution {
    solutions: Vec<TypedTransaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolverRequestResponse {
    pub user_reqs: Vec<UserReq>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllUsersRequestResponse {
    pub all_reqs: HashMap<Address, Vec<UserReq>>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct UserReq {
    pub id: u64,
    pub solved: bool,
    pub user_addr: Address,
    pub hold_token: Address,
    pub hold_amt: u64,
    pub want_token: Address,
    pub solvers: HashMap<Address, SolverSolution>,
    pub winning_solver: Option<Address>,
    pub winning_solution: Option<SolverSolution>,
    pub proof: Option<String>,
}

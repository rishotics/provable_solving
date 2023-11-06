use ethers::types::{Address, Bytes};
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
    pub solutions: Vec<Bytes>,
}

impl SolverSolution {
    pub fn new(tx: Vec<Bytes>) -> Self {
        Self { solutions: tx }
    }
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

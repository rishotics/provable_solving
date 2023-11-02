use crate::Error;
use async_trait::async_trait;
use ethers::types::{transaction::eip1559::Eip1559TransactionRequest, Address};
use hashbrown::HashMap;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use parking_lot::RwLock;
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
    solutions: Vec<Eip1559TransactionRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolverRequestResponse {
    pub user_reqs: Vec<UserReq>,
}

#[derive(Serialize, Clone)]
pub struct AllUsersRequestResponse {
    all_reqs: HashMap<Address, Vec<UserReq>>,
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

#[derive(Debug, Default)]
pub struct AuctioneerApiImpl {
    pub user_reqs: RwLock<HashMap<Address, Vec<UserReq>>>,
}

#[rpc(server, namespace = "auction")]
trait AuctionApi {
    #[method(name = "getReq")]
    fn get_req(&self, user_addr: Address) -> RpcResult<SolverRequestResponse>;

    #[method(name = "getAllReq")]
    fn get_all_reqs(&self) -> RpcResult<AllUsersRequestResponse>;

    #[method(name = "getStatus")]
    fn get_status(&self) -> RpcResult<bool>;

    #[method(name = "populateUser")]
    fn populate_user(&self, user_addr: Address, reqs: Vec<UserReq>) -> RpcResult<bool>;

    #[method(name = "sendSolutions")]
    fn send_solutions(
        &self,
        solver_addr: Address,
        solver_solutions: SolverSolution,
        user_addr: Address,
        user_req: UserReq,
    ) -> RpcResult<UserReq>;
}

impl AuctioneerApiImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl AuctionApiServer for AuctioneerApiImpl {
    fn get_req(&self, user_addr: Address) -> RpcResult<SolverRequestResponse> {
        match self.user_reqs.read().get(&user_addr) {
            Some(user_reqs) => {
                let response = SolverRequestResponse {
                    user_reqs: user_reqs.clone(),
                };
                Ok(response)
            }
            None => Err(Error::UserNotFound("User not found".to_string()).into()),
        }
    }

    fn get_all_reqs(&self) -> RpcResult<AllUsersRequestResponse> {
        let all_reqs = self.user_reqs.read().clone();
        let response = AllUsersRequestResponse { all_reqs };
        Ok(response)
    }

    fn populate_user(&self, user_addr: Address, reqs: Vec<UserReq>) -> RpcResult<bool> {
        self.user_reqs.write().insert(user_addr, reqs);
        Ok(true)
    }

    fn send_solutions(
        &self,
        solver_addr: Address,
        solutions: SolverSolution,
        user_addr: Address,
        user_req: UserReq,
    ) -> RpcResult<UserReq> {
        // Updates the user reqs map
        // - find the req in the user_reqs map
        // - update the solvers with solver addresses and solutions
        let mut user_reqs_vec = self.user_reqs.write();

        match user_reqs_vec.get_mut(&user_addr) {
            Some(reqs) => match reqs.iter_mut().find(|req| req.id == user_req.id) {
                Some(matching_req) => {
                    if !matching_req.solved {
                        matching_req.solvers.insert(solver_addr, solutions);
                        Ok(matching_req.clone())
                    } else {
                        Err(Error::UserRequestHasBeenSolved(
                            "User request has been solved".to_string(),
                        )
                        .into())
                    }
                }
                None => Err(Error::UserNotFound("User request not found".to_string()).into()),
            },
            None => Err(Error::UserNotFound("User not found".to_string()).into()),
        }
    }

    fn get_status(&self) -> RpcResult<bool> {
        Ok(true)
    }
}

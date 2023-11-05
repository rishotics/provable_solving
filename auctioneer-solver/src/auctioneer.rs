use crate::types::{AllUsersRequestResponse, SolverRequestResponse, SolverSolution, UserReq};
use crate::Error;
use async_trait::async_trait;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::{Address, Bytes};
use hashbrown::HashMap;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use parking_lot::RwLock;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AuctioneerApiImpl {
    pub user_reqs: RwLock<HashMap<Address, Vec<UserReq>>>,
    provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Default for AuctioneerApiImpl {
    fn default() -> Self {
        AuctioneerApiImpl {
            user_reqs: RwLock::new(HashMap::new()),
            provider: Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from("https://eth.llamarpc.com")
                    .expect("could not instantiate HTTP Provider"),
                // Default to Anvil's testing wallet
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                    .parse::<LocalWallet>()
                    .unwrap(),
            )),
        }
    }
}

#[rpc(server, namespace = "auction")]
trait AuctionApi {
    #[method(name = "sendUserReq")]
    async fn send_user_req(
        &self,
        user: Address,
        hold_token: Address,
        hold_amt: u64,
        want_token: Address,
        tx: Bytes,
    ) -> RpcResult<UserReq>;

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
    pub fn new(rpc_url: String) -> Self {
        return Self {
            user_reqs: RwLock::new(HashMap::new()),
            provider: Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(rpc_url).expect("could not instantiate HTTP Provider"),
                // Default to Anvil's testing wallet
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                    .parse::<LocalWallet>()
                    // .unwrap() for pure convinience :)
                    .unwrap(),
            )),
        };
    }
}

#[async_trait]
impl AuctionApiServer for AuctioneerApiImpl {
    /// Lets the user sends the user request and hold_amt to the auctioneer
    /// and returns the user request
    /// Note: For simplicity, we'll have the user send the hold_amt to the auctioneer directly
    /// so that when the solver successfully solve a request, the auctioneer could pay the solver
    /// from his/her own account
    async fn send_user_req(
        &self,
        user_addr: Address,
        hold_token: Address,
        hold_amt: u64,
        want_token: Address,
        tx: Bytes,
    ) -> RpcResult<UserReq> {
        let receipt = self
            .provider
            .send_raw_transaction(tx)
            .await
            .map_err(|e| Error::SendingTxError(e.to_string()))?
            .log_msg("Transaction broadcasted, pending confirmation")
            .await
            .map_err(|e| Error::SendingTxError(e.to_string()))?;
        log::info!("Transaction confirmed: {:?}", receipt);

        let mut user_req = UserReq::default();
        user_req.user_addr = user_addr;
        user_req.hold_token = hold_token;
        user_req.hold_amt = hold_amt;
        user_req.want_token = want_token;

        self.user_reqs
            .write()
            .insert(user_addr, vec![user_req.clone()]);
        Ok(user_req)
    }

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

    /// Updates the user reqs map
    /// - find the req in the user_reqs map
    /// - update the solvers with solver addresses and solutions
    fn send_solutions(
        &self,
        solver_addr: Address,
        solutions: SolverSolution,
        user_addr: Address,
        user_req: UserReq,
    ) -> RpcResult<UserReq> {
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

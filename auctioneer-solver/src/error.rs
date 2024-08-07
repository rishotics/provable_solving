use jsonrpsee::types::ErrorObject;
use jsonrpsee::types::ErrorObjectOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    UserNotFound(String),
    UserAlreadyExists(String),
    UserRequestHasBeenSolved(String),
    SendingTxError(String),
    SimulatingTxError(String),
    SignningError(String),
}

const USER_NOT_FOUND: i32 = -32601;
const USER_ALREADY_EXISTS: i32 = -32602;
const USER_REQ_HAS_BEEN_SOLVED: i32 = -32603;
const SENDING_TX_ERROR: i32 = -32604;
const SIMULATING_TX_ERROR: i32 = -32605;
const SIGNING_ERROR: i32 = -32606;

impl From<Error> for ErrorObjectOwned {
    fn from(err: Error) -> Self {
        match err {
            Error::UserNotFound(err) => ErrorObject::owned(USER_NOT_FOUND, err, None::<bool>),
            Error::UserAlreadyExists(err) => {
                ErrorObject::owned(USER_ALREADY_EXISTS, err, None::<bool>)
            }
            Error::UserRequestHasBeenSolved(err) => {
                ErrorObject::owned(USER_REQ_HAS_BEEN_SOLVED, err, None::<bool>)
            }
            Error::SendingTxError(err) => ErrorObject::owned(SENDING_TX_ERROR, err, None::<bool>),
            Error::SimulatingTxError(err) => {
                ErrorObjectOwned::owned(SIMULATING_TX_ERROR, err, None::<bool>)
            }
            Error::SignningError(err) => ErrorObjectOwned::owned(SIGNING_ERROR, err, None::<bool>),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

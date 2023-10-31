use jsonrpsee::types::ErrorObject;
use jsonrpsee::types::ErrorObjectOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    UserNotFound(String),
}

const USER_NOT_FOUND: i32 = -32602;

impl From<Error> for ErrorObjectOwned {
    fn from(err: Error) -> Self {
        match err {
            Error::UserNotFound(err) => ErrorObject::owned(USER_NOT_FOUND, err, None::<bool>),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

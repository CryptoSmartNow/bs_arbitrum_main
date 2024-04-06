use alloy_sol_types::{sol, SolError};
use std::string::FromUtf8Error;

sol! {
    error UserNotExist();
    error InvalidPrice();
    error GeneralError();
}

pub enum BitsaveErrors {
    UserNotExist(UserNotExist),
    GeneralError(GeneralError),
    FromUtf8Error(FromUtf8Error),
}

impl Into<Vec<u8>> for BitsaveErrors {
    fn into(self) -> Vec<u8> {
        match self {
            Self::UserNotExist(err) => err.encode(),
            Self::GeneralError(err) => err.encode(),
            Self::FromUtf8Error(err) => err.into_bytes(),
        }
    }
}

impl From<FromUtf8Error> for BitsaveErrors {
    fn from(err: FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}

pub type BResult<T, E = BitsaveErrors> = core::result::Result<T, E>;

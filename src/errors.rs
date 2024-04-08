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

impl From<BitsaveErrors> for Vec<u8> {
    fn from(val: BitsaveErrors) -> Self {
        match val {
            BitsaveErrors::UserNotExist(err) => err.encode(),
            BitsaveErrors::GeneralError(err) => err.encode(),
            BitsaveErrors::FromUtf8Error(err) => err.into_bytes(),
        }
    }
}

impl From<FromUtf8Error> for BitsaveErrors {
    fn from(err: FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}

pub type BResult<T, E = BitsaveErrors> = core::result::Result<T, E>;

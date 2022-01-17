use std::{env::VarError, fmt::Debug, num::ParseIntError};

use codec::Decode;
use dotenv::Error as DotenvError;
use subxt::{sp_runtime::AccountId32, Error as SubxtError};

#[derive(Debug)]
pub enum Error {
    Dotenv(DotenvError),
    Env(VarError),
    ParseInt(ParseIntError),
    Client(String),
    Subxt(SubxtError),
    Custom(String),
}

#[derive(Decode)]
pub struct MemberBanned(pub AccountId32, pub u64, pub Vec<u8>);

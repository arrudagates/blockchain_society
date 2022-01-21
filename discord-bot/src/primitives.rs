use std::{env::VarError, fmt::Debug, num::ParseIntError, string::FromUtf8Error};

use codec::{Decode, Error as DecodeError};
use dotenv::Error as DotenvError;
use serenity::Error as SerenityError;
use subxt::{sp_runtime::AccountId32, Error as SubxtError};

#[derive(Debug)]
pub enum Error {
    Dotenv(DotenvError),
    Env(VarError),
    ParseInt(ParseIntError),
    Client(String),
    Subxt(SubxtError),
    Custom(String),
    Serenity(SerenityError),
    UTF8(FromUtf8Error),
    Decode(DecodeError),
}

// TEMP manually reimplement some events due to decoding issues with Discord IDs
#[derive(Decode)]
pub struct MemberBanned(pub AccountId32, pub u64, pub Vec<u8>);
#[derive(Decode)]
pub struct RoleAssigned(pub AccountId32, pub u64, pub Vec<u8>);

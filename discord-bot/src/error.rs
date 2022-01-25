use std::{env::VarError, fmt::Debug, num::ParseIntError, string::FromUtf8Error};

use codec::Error as DecodeError;
use dotenv::Error as DotenvError;
use serenity::Error as SerenityError;
use subxt::BasicError;

#[derive(Debug)]
pub enum Error {
    Dotenv(DotenvError),
    Env(VarError),
    ParseInt(ParseIntError),
    Client(String),
    Subxt(BasicError),
    Custom(String),
    Serenity(SerenityError),
    UTF8(FromUtf8Error),
    Decode(DecodeError),
}

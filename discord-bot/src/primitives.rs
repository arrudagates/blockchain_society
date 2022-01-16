use std::{env::VarError, fmt::Debug, num::ParseIntError};

use dotenv::Error as DotenvError;
use subxt::Error as SubxtError;

#[derive(Debug)]
pub enum Error {
    Dotenv(DotenvError),
    Env(VarError),
    ParseInt(ParseIntError),
    Client(String),
    Subxt(SubxtError),
}

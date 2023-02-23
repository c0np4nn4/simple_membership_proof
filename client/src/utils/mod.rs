use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) mod circuit;
pub(crate) mod proof;

#[derive(Serialize, Deserialize, Clone)]
pub struct Req<T: Serialize + Clone> {
    pub(crate) name: String,
    pub(crate) method: String,
    pub(crate) body: T,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

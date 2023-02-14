use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;
use tui::widgets::ListState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Request<T: Serialize + Clone> {
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

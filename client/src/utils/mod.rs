use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;
use tui::widgets::ListState;

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;

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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;

    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .method("GET")
        .header(hyper::header::HOST, authority.as_str())
        .body("{\"account_id\": 1}".to_string())?;

    let mut res = sender.send_request(req).await?;

    log::info!(" Response: {}", res.status());
    log::info!(" Headers: {:#?}\n", res.headers());
    log::info!(" Body: {:#?}\n", res.body());

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(&chunk).await?;
        }
    }

    log::info!(" fetch_url Done!");

    Ok(())
}

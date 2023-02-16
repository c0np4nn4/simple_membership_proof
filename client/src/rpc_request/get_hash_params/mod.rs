use super::Result;

use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::Request;
use tokio::net::TcpStream;

use tokio::io::{self, AsyncWriteExt as _};

pub async fn get_hash_params(url: hyper::Uri) -> Result<Vec<String>> {
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

    log::warn!(" Response: {}", res.status());
    log::warn!(" Headers: {:#?}\n", res.headers());
    log::warn!(" Body: {:#?}\n", res.body());

    let mut a: Bytes = Bytes::default();

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            a = chunk.to_owned();
            // io::stdout().write_all(&chunk).await?;
            log::info!("result: {:?}", &chunk.to_vec()[0]);
        }
    }

    let hash_params: Vec<String> =
        serde_json::from_slice(&a).unwrap_or(vec!["NO_DATA".to_string()]);

    for i in 0..(hash_params.len()) {
        log::info!(" hash_params[{}] [0..16]: {:?}", i, &hash_params[i]);
    }
    //
    log::info!(" get_hash_params done!");

    Ok(hash_params)
}

use super::Result;

use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::Request;
use tokio::net::TcpStream;

pub async fn get_path(url: hyper::Uri) -> Result<Vec<Vec<u8>>> {
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

    let mut a: Bytes = Bytes::default();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            a = chunk.clone();
            // io::stdout().write_all(&chunk).await?;
        }
    }

    let path: Vec<Vec<u8>> = serde_json::from_slice(&a).unwrap();

    log::info!(" get_path done!");

    Ok(path)
}

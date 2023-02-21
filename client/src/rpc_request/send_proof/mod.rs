use crate::utils::circuit::{MerkleConfig, Root};
use crate::utils::proof::gen_proof_and_vk;

use super::Result;

use ark_crypto_primitives::Path;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::Request;
use serde::Serialize;
use tokio::net::TcpStream;

use tokio::io::{self, AsyncWriteExt as _};

#[derive(Serialize)]
struct SendProofRequest {
    proof: Vec<u8>,
    public_input: Vec<u8>,
    vk: Vec<u8>,
}

pub async fn send_proof(url: hyper::Uri, path: &Vec<Vec<u8>>, root: &Vec<u8>) -> Result<String> {
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

    // for test, leaf == 1 as u8
    let leaf = 1u8;

    let root = Root::deserialize(root.as_slice()).unwrap();

    log::info!("root: {:?}", root);

    let mut paths: Vec<Path<MerkleConfig>> = Vec::default();

    for p in path {
        let res = Path::<MerkleConfig>::deserialize(p.as_slice()).unwrap();

        log::info!("path: {:?}", res.leaf_sibling_hash);

        paths.push(res);
    }

    gen_proof_and_vk(leaf, root, paths[leaf as usize].clone());
    // a.serialize_uncompressed(writer)
    // gen_proof_and_vk(1);

    // gen_proof
    // public_input as root
    // gen vk

    // req body

    let req_body =
        stringify!({"proof": [1, 2, 3, 4],"public_input": [10, 20, 30, 40],"vk": [5, 6, 7, 8]})
            .to_string();

    log::info!("[!] req_body: {:#?}", req_body);

    let req = Request::builder()
        .uri(url)
        .method("POST")
        .header(hyper::header::HOST, authority.as_str())
        .body(req_body)?;

    let mut res = sender.send_request(req).await?;

    let mut a: Bytes = Bytes::default();

    while let Some(next) = res.frame().await {
        let frame = next?;

        if let Some(chunk) = frame.data_ref() {
            a = chunk.to_owned();
            // io::stdout().write_all(&chunk).await?;
            log::info!("result: {:?}", chunk);
        }
    }

    // let hash_params: Vec<String> =
    //     serde_json::from_slice(&a).unwrap_or(vec!["NO_DATA".to_string()]);

    // for i in 0..(hash_params.len()) {
    //     log::info!(" hash_params[{}] [0..16]: {:?}", i, &hash_params[i]);
    // }
    // //
    // log::info!(" get_hash_params done!");

    Ok(String::from("not yet"))
}

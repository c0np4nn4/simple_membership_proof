use crate::utils::circuit::{MerkleConfig, Root};
use crate::utils::proof::gen_proof_and_vk;

use super::Result;

use ark_crypto_primitives::Path;
use ark_ff::biginteger::BigInteger256;
use ark_serialize::CanonicalDeserialize;
use http_body_util::BodyExt;
use hyper::Request;
use serde::Serialize;
use tokio::net::TcpStream;

#[derive(Serialize)]
struct SendProofRequest {
    proof: Vec<u8>,
    public_input: Vec<u8>,
    vk: Vec<u8>,
}

pub async fn send_proof(
    url: hyper::Uri,
    path: &Vec<Vec<u8>>,
    root: &Vec<u8>,
    leaf: u8,
    leaf_idx: u8,
) -> Result<()> {
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
    // let leaf = 1u8;

    let ser_root = root.clone();

    let root = convert_u8_vec_to_u64_array(root.clone());
    let root = Root::new(BigInteger256::new(root));

    log::info!("root: {:?}", root);

    let mut paths: Vec<Path<MerkleConfig>> = Vec::default();

    for p in path {
        let res = Path::<MerkleConfig>::deserialize(p.as_slice()).unwrap();

        log::info!("path: {:?}", res.leaf_sibling_hash);

        paths.push(res);
    }

    let (ser_vk, ser_proof): (Vec<u8>, Vec<u8>) =
        gen_proof_and_vk(leaf, root, paths[leaf_idx as usize].clone());

    let req_body = SendProofRequest {
        proof: ser_proof,
        vk: ser_vk,
        public_input: ser_root,
    };

    let req_body = serde_json::to_string(&req_body).unwrap();

    let req = Request::builder()
        .uri(url)
        .method("POST")
        .header(hyper::header::HOST, authority.as_str())
        .body(req_body)?;

    let mut res = sender.send_request(req).await?;

    while let Some(next) = res.frame().await {
        let frame = next?;

        if let Some(chunk) = frame.data_ref() {
            // io::stdout().write_all(&chunk).await?;
            log::info!("result: {:?}", chunk);
        }
    }

    Ok(())
}

fn convert_u8_vec_to_u64_array(vec_u8: Vec<u8>) -> [u64; 4] {
    let mut res: [u64; 4] = [0x0u64; 4];

    for i in 0..32 {
        let idx = i / 8;
        res[idx] <<= 8;
        res[idx] ^= vec_u8[i] as u64;

        // println!("[1] vec_u8: {:02x?}", vec_u8[i]);
        // println!("[2] res: {:016x?}", res[idx]);
    }

    // println!("[!] check");

    // println!("res: {:?}", res);

    res
}

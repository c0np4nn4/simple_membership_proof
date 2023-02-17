use crate::{payment::account::AccountId, Context, Response, TREE_SIZE};
use ark_crypto_primitives::merkle_tree;
use ark_serialize::CanonicalSerialize;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetTreeRequest {
    account_id: u8,
}

pub async fn get_tree(mut ctx: Context) -> Response {
    let body: GetTreeRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap();
        }
    };

    println!("body: {:#?}", body);

    let state = ctx.state.state_thing;
    let acc_id = AccountId(body.account_id);
    let state_lock = state.lock().unwrap();

    let merkle_tree = state_lock.account_merkle_tree.clone();

    let mut paths: Vec<Vec<u8>> = vec![];

    for i in 0..(TREE_SIZE / 2) {
        let mut path_bytes: Vec<u8> = vec![];

        let path = merkle_tree.generate_proof(i as usize).unwrap();

        path.serialize(&mut path_bytes).unwrap();

        paths.push(path_bytes);
    }

    let paths = serde_json::to_string(&paths).unwrap();

    Response::new(paths.into())
}

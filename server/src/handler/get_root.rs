use crate::{payment::account::AccountId, Context, Response, TREE_SIZE};
use ark_crypto_primitives::merkle_tree;
use ark_serialize::CanonicalSerialize;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetRootRequest {
    account_id: u8,
}

pub async fn get_root(mut ctx: Context) -> Response {
    let body: GetRootRequest = match ctx.body_json().await {
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

    // let mut paths: Vec<Vec<u8>> = vec![];
    let root = merkle_tree.root().to_string();

    let root = serde_json::to_vec(&root).unwrap();

    Response::new(root.into())
}

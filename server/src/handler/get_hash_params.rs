use crate::{payment::account::AccountId, Context, Response, TREE_SIZE};
use ark_crypto_primitives::merkle_tree;
use ark_serialize::CanonicalSerialize;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetTreeRequest {
    account_id: u8,
}

pub async fn get_hash_params(mut ctx: Context) -> Response {
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

    let mut hash_params: Vec<String> = Vec::default();

    hash_params.push(format!("{:?}", state_lock.leaf_crh_params));

    hash_params.push(format!("{:?}", state_lock.two_to_one_crh_params));

    // println!("hash_params: {:?}", hash_params);

    let hash_params = serde_json::to_string(&hash_params).unwrap();

    Response::new(hash_params.into())
}

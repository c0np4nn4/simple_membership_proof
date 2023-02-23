use crate::{
    payment::{account::AccountId, ledger::MerkleConfig},
    Context, Response, TREE_SIZE,
};
use ark_crypto_primitives::Path;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetTreeRequest {
    account_id: u8,
}

pub async fn get_path(mut ctx: Context) -> Response {
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
    let _acc_id = AccountId(body.account_id);
    let state_lock = state.lock().unwrap();

    // let merkle_tree = state_lock.account_merkle_tree.clone();
    let merkle_tree = state_lock.clone();

    let mut paths: Vec<Vec<u8>> = vec![];

    for i in 0..8 {
        let mut path_bytes: Vec<u8> = vec![];

        let path = merkle_tree.generate_proof(i).unwrap();

        if i == 1 {
            println!("path[{}]: {:?}", i, path.auth_path);
        }

        path.serialize(&mut path_bytes).unwrap();

        println!(
            "path byte check[{}]: {:02x?}",
            i,
            &path_bytes.as_slice()[0..8]
        );

        // for test
        {
            let tmp = Path::<MerkleConfig>::deserialize(path_bytes.as_slice()).unwrap();

            assert_eq!(tmp.auth_path, path.auth_path);
        }

        paths.push(path_bytes);
    }

    let paths = serde_json::to_string(&paths).unwrap();

    Response::new(paths.into())
}

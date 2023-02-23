use crate::{Context, Response};
use ark_serialize::CanonicalSerialize;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetTreeRequest {
    account_id: u8,
}

pub async fn get_path(mut ctx: Context) -> Response {
    let _body: GetTreeRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap();
        }
    };

    let state = ctx.state.state_thing;
    let state_lock = state.lock().unwrap();

    let merkle_tree = state_lock.clone();

    let mut paths: Vec<Vec<u8>> = vec![];

    for i in 0..8 {
        let mut path_bytes: Vec<u8> = vec![];

        let path = merkle_tree.generate_proof(i).unwrap();

        path.serialize(&mut path_bytes).unwrap();

        paths.push(path_bytes);
    }

    let paths = serde_json::to_string(&paths).unwrap();

    Response::new(paths.into())
}

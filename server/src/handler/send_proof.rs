use crate::{
    payment::{account::AccountId, ledger::Parameters},
    Context, Response, TREE_SIZE,
};
use ark_std::test_rng;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct SendProofRequest {
    proof: Vec<u8>,
    public_input: Vec<u8>,
    vk: Vec<u8>,
}

pub async fn send_proof(mut ctx: Context) -> Response {
    let body: SendProofRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap();
        }
    };

    let state = ctx.state.state_thing;
    let mut state_lock = state.try_lock().unwrap();

    if state_lock.next_available_account.unwrap() == AccountId(TREE_SIZE / 2) {
        return Response::new(format!("[-] Overflow Detected (Maximum: {})\n", TREE_SIZE).into());
    }

    let mut rng = test_rng();

    let pp = Parameters::sample(&mut rng);

    // encrypt(alice_pk, user_ecc_pk)
    // encrypt(alice_sk, user_ecc_pk)

    Response::new(format!("validation done, not implemented yet").into())
}

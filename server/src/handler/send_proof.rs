use crate::{circuit::Root, Context, Response};
use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::SNARK;
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
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

    println!("[!] proof: {:?}", body.proof);
    println!("[!] public_input: {:?}", body.public_input);
    println!("[!] vk: {:?}", body.vk);

    let state = ctx.state.state_thing;
    let state_lock = state.try_lock().unwrap();

    let mut _rng = test_rng();

    // --------------------------
    let proof = Proof::<Bls12_381>::deserialize(body.proof.as_slice()).unwrap();

    let public_input = convert_u8_vec_to_u64_array(body.public_input.clone());
    let public_input = Root::new(ark_ff::BigInteger256(public_input));

    let vk = VerifyingKey::<Bls12_381>::deserialize(body.vk.as_slice()).unwrap();

    let mut tmp1 = Vec::new();
    let mut tmp2 = Vec::new();
    proof.serialize(&mut tmp1).unwrap();
    vk.serialize_unchecked(&mut tmp2).unwrap();

    println!("[check]");
    println!("  proof: {:?}", tmp1);
    println!("  vk: {:?}", tmp2);
    println!("  root: {:?}", public_input);

    let valid_proof = Groth16::verify(&vk, &[public_input], &proof).unwrap();

    if valid_proof == true {
        Response::new(format!("validation done, Good").into())
    } else {
        Response::new(format!("validation done, Fail").into())
    }
}

fn convert_u8_vec_to_u64_array(vec_u8: Vec<u8>) -> [u64; 4] {
    let mut res: [u64; 4] = [0x0u64; 4];

    for i in 0..32 {
        let idx = i / 8;
        res[idx] <<= 8;
        res[idx] ^= vec_u8[i] as u64;
    }

    res
}

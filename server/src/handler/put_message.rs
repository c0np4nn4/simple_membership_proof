// use crate::{
//     payment::{account::AccountId, ledger::Parameters},
//     Context, Response, TREE_SIZE,
// };
// use ark_std::test_rng;
// use hyper::StatusCode;
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// struct PutMessageRequest {
//     _user_name: String,
//     user_ecc_pk: String,
// }

// pub async fn put_message(mut ctx: Context) -> Response {
//     let body: PutMessageRequest = match ctx.body_json().await {
//         Ok(v) => v,
//         Err(e) => {
//             return hyper::Response::builder()
//                 .status(StatusCode::BAD_REQUEST)
//                 .body(format!("could not parse JSON: {}", e).into())
//                 .unwrap();
//         }
//     };

//     let state = ctx.state.state_thing;
//     let mut state_lock = state.try_lock().unwrap();

//     if state_lock.next_available_account.unwrap() == AccountId(TREE_SIZE / 2) {
//         return Response::new(format!("[-] Overflow Detected (Maximum: 32)\n").into());
//     }

//     let mut rng = test_rng();

//     let pp = Parameters::sample(&mut rng);

//     let (new_acc_id, alice_pk, alice_sk) =
//         state_lock.sample_keys_and_register(&pp, &mut rng).unwrap();

//     // encrypt(alice_pk, user_ecc_pk)
//     // encrypt(alice_sk, user_ecc_pk)

//     Response::new(
//         format!(
//             "[+] encrypted pk: {:?}\n[+] encrypted sk: {:?}\n[+] next_available acc_id: {:?}\n",
//             // "encrypted pk".to_string(),"encrypted sk".to_string()
//             alice_pk,
//             alice_sk,
//             state_lock.next_available_account
//         )
//         .into(),
//     )
// }

use std::sync::{Arc, Mutex};

use crate::{payment::account::AccountId, Context, Response};
use hyper::{StatusCode, header::TE};
use serde::Deserialize;
use serde::Serialize;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct GetBalanceRequest {
    account_id: u8,
}

// for test
#[derive(Serialize, Deserialize, Debug)]
struct Testing {
    test_code: i32,
    test_str: String,
}

pub async fn get_balance(mut ctx: Context) -> Response {
    let body: GetBalanceRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap();
        }
    };

    // 자동으로 입력 걸러주는지 확인
    println!("body: {:?}", body);

    let state = ctx.state.state_thing;
    let acc_id = AccountId(body.account_id);
    let state_lock = state.lock().unwrap();

    let b = match state_lock.id_to_account_info.get(&acc_id) {
        Some(b) => b,
        None => {
            return Response::new(
                format!("[-] Error: Cannot find account_id: {:?}\n", body.account_id).into(),
            );
        }
    };

    // struct serializing 실습
    // https://blog.majecty.com/posts/2018-12-31-a-rust-serde-derive-value.html
    let t = Testing {
        test_code: 216,
        test_str: "jeong".to_string(),
    };

    let serialized_t = serde_json::to_string(&t).unwrap();

    Response::new(
        format!(
            "[+] get_balance, account_id: {:?}, balance: {:?}\n",
            // body.account_id, b.balance.0
            body.account_id, serialized_t
            
        )
        .into(),
    )
}

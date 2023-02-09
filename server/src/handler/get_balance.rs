use crate::{payment::account::AccountId, Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetBalanceRequest {
    account_id: u8,
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

    Response::new(
        format!(
            "[+] get_balance, account_id: {:?}, balance: {:?}\n",
            body.account_id, b.balance.0
        )
        .into(),
    )
}

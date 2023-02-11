use crate::{
    payment::{account::AccountId, ledger::Amount},
    Context, Response,
};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct AddBalanceRequest {
    account_id: u8,
    amount: u64,
}

pub async fn add_balance(mut ctx: Context) -> Response {
    let body: AddBalanceRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    let state = ctx.state.state_thing;
    let acc_id = AccountId(body.account_id);
    let mut state_lock = state.lock().unwrap();

    let b = match state_lock.id_to_account_info.get(&acc_id) {
        Some(b) => b.balance.0.clone(),
        None => {
            return Response::new(
                format!("[-] Error: Cannot find account_id: {:?}\n", body.account_id).into(),
            );
        }
    };

    state_lock.update_balance(acc_id, Amount(b + body.amount));

    Response::new(
        format!(
            "[+] add_balance, account_id: {:?}, new_balance: {:?}\n",
            // body.account_id, ctx.state.state_thing.lock().unwrap().id_to_account_info.get(&account_id).unwrap().balance.0
            body.account_id,
            state_lock
                .id_to_account_info
                .get(&acc_id)
                .unwrap()
                .balance
                .0,
        )
        .into(),
    )
}

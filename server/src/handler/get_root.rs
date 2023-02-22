use crate::{payment::account::AccountId, Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetRootRequest {
    account_id: u8,
}

fn convert_u64_array_to_u8_vec(array_u64: [u64; 4]) -> Vec<u8> {
    let mut res = Vec::<u8>::new();

    let mask: [u64; 8] = [
        0xff00_0000_0000_0000,
        0x00ff_0000_0000_0000,
        0x0000_ff00_0000_0000,
        0x0000_00ff_0000_0000,
        0x0000_0000_ff00_0000,
        0x0000_0000_00ff_0000,
        0x0000_0000_0000_ff00,
        0x0000_0000_0000_00ff,
    ];

    for i in 0..4 {
        let a = array_u64[i];
        // println!("a: {:016x?}", a);

        for j in 0..8 {
            let b: u8 = (((a & mask[j]) >> (8 * (7 - j))) & 0xff) as u8;
            // println!("\tb: {:02x?}", b);
            //
            res.push(b);
        }
    }

    res

    // vec![]
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
    let _acc_id = AccountId(body.account_id);
    let state_lock = state.lock().unwrap();

    // let merkle_tree = state_lock.account_merkle_tree.clone();
    let merkle_tree = state_lock.clone();

    // let mut root_vec = Vec::default();

    println!("[1] root: {:?}", merkle_tree.root());
    println!("[2] root: {:?}", merkle_tree.root().0);
    println!("[3] root: {:?}", merkle_tree.root().0 .0);

    // merkle_tree.root().0 .0.serialize(&mut root_vec).unwrap();

    let root_vec = convert_u64_array_to_u8_vec(merkle_tree.root().0 .0);

    // println!("root: {:?}", root_vec);

    Response::new(root_vec.into())
}

use crate::{payment::account::AccountId, Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetRootRequest {
    account_id: u8,
}

fn convert_u64_array_to_u8_vec(array_u64: [u64; 4]) -> Vec<u8> {
    // use std::ptr;
    // use std::slice;
    // let slice_u64 = &array_u64[..];
    // let num_bytes = slice_u64.len() * std::mem::size_of::<u64>();
    // let mut vec_u8: Vec<u8> = Vec::with_capacity(num_bytes);
    // let ptr_u64 = slice_u64.as_ptr();
    // unsafe {
    //     let ptr_u8 = vec_u8.as_mut_ptr();
    //     let slice_u8 = slice::from_raw_parts_mut(ptr_u8, num_bytes);
    //     ptr::copy_nonoverlapping(ptr_u64 as *const u8, slice_u8.as_mut_ptr(), num_bytes);
    //     vec_u8.set_len(num_bytes);
    // }
    // vec_u8
    let mut res = Vec::<u8>::new();

    for i in 0..4 {
        let a = array_u64[i];
        println!("a: {:064x?}", a);

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

        for j in 0..8 {
            let b: u8 = (((a & mask[j]) >> (8 * (7 - i))) & 0xff) as u8;
            println!("b: {:02x?}", b);
        }
    }

    vec![]
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

    let merkle_tree = state_lock.account_merkle_tree.clone();

    // let mut root_vec = Vec::default();

    println!("[1] root: {:?}", merkle_tree.root());
    println!("[2] root: {:?}", merkle_tree.root().0);
    println!("[3] root: {:?}", merkle_tree.root().0 .0);

    // merkle_tree.root().0 .0.serialize(&mut root_vec).unwrap();
    // merkle_tree.root()의 타입 궁금해서 찍어봄
    // let root = merkle_tree.root();

    let root_vec = convert_u64_array_to_u8_vec(merkle_tree.root().0 .0);

    // println!("root: {:?}", root_vec);

    Response::new(root_vec.into())
}

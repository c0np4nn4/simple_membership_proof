use crate::{payment::account::AccountId, Context, Response, TREE_SIZE};
use ark_crypto_primitives::merkle_tree;
use ark_serialize::CanonicalSerialize;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetRootRequest {
    account_id: u8,
}

// fn convert_u64_to_u8(vec_u64: Vec<u64>) -> Vec<u8> {
//     use std::mem;
//     let num_bytes = vec_u64.len() * mem::size_of::<u64>();
//     let mut vec_u8: Vec<u8> = Vec::with_capacity(num_bytes);
//     unsafe {
//         vec_u8.set_len(num_bytes);
//         let ptr_u64 = vec_u64.as_ptr();
//         let ptr_u8 = vec_u8.as_mut_ptr();
//         // copy the bytes from the u64 slice to the u8 slice
//         std::ptr::copy_nonoverlapping(ptr_u64, ptr_u8 as *mut u64, num_bytes);
//     }
//     vec_u8
// }

fn convert_u64_array_to_u8_vec(array_u64: [u64; 4]) -> Vec<u8> {
    use std::ptr;
    use std::slice;
    let slice_u64 = &array_u64[..];
    let num_bytes = slice_u64.len() * std::mem::size_of::<u64>();
    let mut vec_u8: Vec<u8> = Vec::with_capacity(num_bytes);
    let ptr_u64 = slice_u64.as_ptr();
    unsafe {
        let ptr_u8 = vec_u8.as_mut_ptr();
        let slice_u8 = slice::from_raw_parts_mut(ptr_u8, num_bytes);
        ptr::copy_nonoverlapping(ptr_u64 as *const u8, slice_u8.as_mut_ptr(), num_bytes);
        vec_u8.set_len(num_bytes);
    }
    vec_u8
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

    let root_vec = convert_u64_array_to_u8_vec(merkle_tree.root().0 .0);

    // println!("root: {:?}", root_vec);

    Response::new(root_vec.into())
}

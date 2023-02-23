// use crate::payment::ledger::{MerkleConfig, Parameters};
// use ark_crypto_primitives::MerkleTree;
// use ark_std::test_rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) mod circuit;
pub(crate) mod proof;

#[derive(Serialize, Deserialize, Clone)]
pub struct Req<T: Serialize + Clone> {
    pub(crate) name: String,
    pub(crate) method: String,
    pub(crate) body: T,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

// pub fn make_tree() -> MerkleTree<MerkleConfig> {
//     let mut rng = test_rng();
//     let pp = Parameters::sample(&mut rng);

//     let num = 32usize;
//     let height = ark_std::log2(num);

//     let merkle_tree = MerkleTree::blank(
//         &pp.leaf_crh_params,
//         &pp.two_to_one_crh_params,
//         height as usize,
//     )
//     .unwrap();

//     merkle_tree
// }

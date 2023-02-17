extern crate rand;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use sha2::digest::typenum::Length;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use byteorder::{BigEndian, ReadBytesExt};
use sha2::{Digest, Sha256};

use crate::{common::*, SimpleMerkleTree};
use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget, CRH};
use ark_groth16::Groth16;
use ark_snark::SNARK;

use crate::common::{LeafHash, TwoToOneHash};

extern crate serde;
extern crate serde_json;

// 개인정보 구조체 (8개; 임의로 정함)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Privacy {
    pub member_type: u8,
    pub name: String,
    pub sex: u8,
    pub birth_day: u8,
    pub birth_month: u8,
    pub birth_year: u32,
    pub phone_number: String,
    pub e_mail: String,
    pub zip_code: u32,
    pub address: String,
    pub receipt: u8,
}

#[test]
fn own_merkle_tree() {
    fn construct_privacy_list() -> Vec<String> {
        let mut privacy_vec: Vec<String> = Vec::new();

        for i in 0..8 {
            let _member_type = rand::thread_rng().gen_range(1..3);
            let _name = Alphanumeric.sample_string(&mut rand::thread_rng(), 7);
            let _sex = rand::thread_rng().gen_range(3..5);
            let _birth_day = rand::thread_rng().gen_range(1..29);
            let _birth_month = rand::thread_rng().gen_range(1..13);
            let _birth_year = rand::thread_rng().gen_range(1900..2024);
            let _phone_number = rand::thread_rng()
                .gen_range(10000000..100000000)
                .to_string();
            let _e_mail = Alphanumeric.sample_string(&mut rand::thread_rng(), 15);
            let _zip_code = rand::thread_rng().gen_range(10000..1000000);
            let _address = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
            let _receipt = rand::thread_rng().gen_range(0..2);

            let privacy_struct = Privacy {
                member_type: _member_type,
                name: _name,
                sex: _sex,
                birth_day: _birth_day,
                birth_month: _birth_month,
                birth_year: _birth_year,
                phone_number: _phone_number,
                e_mail: _e_mail,
                zip_code: _zip_code,
                address: _address,
                receipt: _receipt,
            };
            let serialized_privacy = serde_json::to_string(&privacy_struct).unwrap();
            privacy_vec.push(serialized_privacy);
        }

        privacy_vec
    }

    fn string_to_hash(privacy_vec: &Vec<String>) -> [u64; 8] {
        let mut privacy_arr: [u64; 8] = [0; 8];
        for j in 0..8 {
            // serialize 된 값을 sha-256으로 변환
            let mut hasher = Sha256::new();
            hasher.update(privacy_vec[j].as_bytes());

            // sha-256 결과값을 u8 array로 변환
            let u8_arr: [u8; 32] = hasher.finalize().as_slice().try_into().expect("Wrong");
            let mut u8_arr1: [u8; 8] = [0; 8];
            let mut u8_arr2: [u8; 8] = [0; 8];
            let mut u8_arr3: [u8; 8] = [0; 8];
            let mut u8_arr4: [u8; 8] = [0; 8];
            for i in 0..8 {
                u8_arr1[i] = u8_arr[i];
            }

            // u8 array를 u64로 변환
            let u8arr_to_int1 = u64::from_le_bytes(u8_arr1);
            privacy_arr[j] = u8arr_to_int1;
        }

        privacy_arr
    }

    fn make_merkle_tree(leaves: &[u64]) -> SimpleMerkleTree {
        let mut rng = ark_std::test_rng();
        let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
        let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

        let tree =
            crate::SimpleMerkleTree::new(&leaf_crh_params, &two_to_one_crh_params, leaves).unwrap();

        tree
    }

    let privacy_vec = construct_privacy_list();
    let sth = string_to_hash(&privacy_vec);
    let merkle_tree = make_merkle_tree(&sth);
}


extern crate rand;
use rand::Rng;
use rand::distributions::{Alphanumeric, DistString};
use pad::{PadStr, Alignment}; // 휴대폰 번호 때문에 패딩하려고 했는데 결과가 String이 아니라고 에러 떠서 사용 안 함
use serde::Deserialize;
use serde::Serialize;

use crate::{common::*, SimpleMerkleTree};
use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget, CRH};

use crate::common::{LeafHash, TwoToOneHash};

extern crate serde;
extern crate serde_json;

// 개인정보 구조체 (16개; 임의로 정함)
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
    pub receipt: u8
}

#[test]
pub fn construct_privacy_list() {
    // let mut privacy_vec: Vec<Privacy> = Vec::new();
    let str = "str".to_owned();
    // String으로 초기화한 array 생성해서 serializing한 struct를 요소로 한
    // array를 생성하고자 했는데 소유권 문제 때문인지 오류 발생
    // let mut privacy_arr = [str.; 16];
    for i in 0..17 {
        let _member_type = rand::thread_rng().gen_range(1..3);
        let _name = Alphanumeric.sample_string(&mut rand::thread_rng(), 7);
        let _sex = rand::thread_rng().gen_range(3..5);
        let _birth_day = rand::thread_rng().gen_range(1..29);
        let _birth_month = rand::thread_rng().gen_range(1..13);
        let _birth_year = rand::thread_rng().gen_range(1900..2024);
        let _phone_number = rand::thread_rng().gen_range(10000000..100000000).to_string();
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
            receipt: _receipt
        };
        // privacy_vec.push(privacy_struct);
        let serialized_privacy = serde_json::to_string(&privacy_struct).unwrap();
        // 이후에 array[i] = serialized_privacy 해서 배열 요소 바꾸고자 했음
    }
    
    // for test
    // for j in 0..17 {
    //     println!("{:?}", privacy_vec[j]);
    // }


}

#[test]
fn make_merkle_tree() {
    let mut rng = ark_std::test_rng();
    let leaf_crh_params 
        = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params 
        = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

    // merkle tree를 생성해보려고 했으나 leaves에 임의의 String array를 넣는 것 실패
    // merkle tree가 제대로 생성됐는지 보고 싶었으나 디버깅 실패
    #[derive(Debug)]
    let tree 
        = crate::SimpleMerkleTree::new(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &[10u8, 20u8, 30u8, 40u8, 50u8, 60u8, 70u8, 80u8]
        ).unwrap();
    
    println!("{:?}", tree);
}
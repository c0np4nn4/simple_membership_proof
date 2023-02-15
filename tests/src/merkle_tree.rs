extern crate rand;
use rand::Rng;
use rand::distributions::{Alphanumeric, DistString};
use pad::{PadStr, Alignment}; // 휴대폰 번호 때문에 패딩하려고 했는데 결과가 String이 아니라고 에러 떠서 사용 안 함
use serde::Deserialize;
use serde::Serialize;

extern crate serde;
extern crate serde_json;

#[derive(Debug)]
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
pub fn main() {
    // 개인정보 구조체 (16개; 임의로 정함)
    let mut privacy_vec: Vec<Privacy> = Vec::new();
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
        privacy_vec.push(privacy_struct);
    }
    
    // for test
    for j in 0..17 {
        println!("{:?}", privacy_vec[j]);
    }
}
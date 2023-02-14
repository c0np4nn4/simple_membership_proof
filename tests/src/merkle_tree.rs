use std::io;
use rand::Rng;

#[derive(Debug)]
pub struct Privacy {
    pub member_type: String,
    pub name: String,
    pub sex: String,
    pub birth_day: u8,
    pub birth_month: u8,
    pub birth_year: u32,
    pub phone_number: String,
    pub e_mail: String,
    pub zip_code: u32,
    pub address: String,
    pub receipt: bool
}

pub fn main() {
    let test = Privacy {
        member_type: "individual".to_string(),
        name: "lacuna".to_string(),
        sex: "female".to_string(),
        birth_day: 16,
        birth_month: 2,
        birth_year: 2002,
        phone_number: "000-0000-0000".to_string(),
        e_mail: "00000@naver.com".to_string(),
        zip_code: 101010,
        address: "000-000".to_string(),
        receipt: true
    };
    let privacy_arr:[Privacy; 1] = [test];
    println!("{:?}", privacy_arr);
}

// rand 계속 에러남
// for문 돌려서 임의의 privacy 생성 예정
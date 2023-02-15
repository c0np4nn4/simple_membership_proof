extern crate rand;
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

#[test]
pub fn main() {
    let privacy_vec: Vec<Privacy> = Vec::new();
    let num = rand::thread_rng().gen_range(1..101);
    println!("{:?}", num);
    
}
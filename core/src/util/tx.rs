#![deny(warnings)]

use serde::{Serialize, Deserialize};

pub fn test(){
    println!("tx");
}

#[derive(Debug,Serialize, Deserialize, Clone,PartialEq)]
pub struct Tx{
    pub address: String,
    pub token: String,
    pub amount: f32,
    pub time: u64,
}

impl Tx {
    pub fn new(address: String, token: String, amount: f32) -> Tx {
        let time:u64 = crate::util::func::timestamp();
        Tx{
            address,
            token,
            amount,
            time
        }
    }

    pub fn to_string(&self) ->String{
        let res = serde_json::to_string(&self).unwrap();
        res
    }
}
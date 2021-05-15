#![deny(warnings)]
use serde::{Serialize, Deserialize};
use crate::util::tx::Tx;

pub fn test(){
    println!("utxo");
}


#[derive(Debug,Serialize, Deserialize,PartialEq, Clone)]
pub struct Utxo{
    pub time:u64,
    pub input:Tx,
    pub output:Tx
}

impl Utxo {
    pub fn new(input: Tx, output:Tx) -> Utxo {
        let time:u64 = crate::util::func::timestamp();
        Utxo{
            time,input,output
        }
    }

    pub fn to_string(&self){
        serde_json::to_string(&self).unwrap();
    }
}
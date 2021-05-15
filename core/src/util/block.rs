use crate::util::utxo::Utxo;
use serde::{Serialize, Deserialize};

use crate::util::tx::Tx;


pub fn test() {
    println!("block");
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Block {
    pub height: u128,
    pub hash: String,
    pub time: u64,
    pub transaction: Utxo,
    pub prehash: String,
}




impl Block {
    pub fn new(utxo: Utxo,last_prehash:&str,last_height:u128) -> Block{
        let height = last_height + 1;
        let prehash = last_prehash.clone().to_string();
        // let prehash = "".into();
        let time: u64 = crate::util::func::timestamp();
        let transaction = utxo;
        let str = format!("{:?}{:?}{:?}{:?}", time.to_string(), height.to_string(), transaction.to_string(), prehash);
        let hash = crate::util::func::sha256(&str);

        Block{
            height,
            hash,
            time,
            transaction,
            prehash,
        }
    }
    pub fn to_string(&self) -> String {
        let res = serde_json::to_string(&self).unwrap();
        res
    }

    pub fn read_block(path: &str) -> Block {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let obj: Block = serde_json::from_reader(reader).unwrap();
        return obj;
    }
}

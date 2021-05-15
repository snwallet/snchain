use crate::util::utxo::Utxo;
use serde::{Serialize, Deserialize};

use lazy_static::lazy_static;
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


lazy_static! {
    #[derive(Debug)]
    pub static ref CHAIN:Vec<Block> = vec![
        Block { height: 1, hash: "6a696516b663720306ceafcabf59361090736d3b763551e5ee755ab15c633665".to_string(), time: 0, transaction: Utxo { time: 0, input: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), amount: 0.0, time: 0 }, output: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".into(), amount: 0.0, time: 0 } }, prehash: "0000000000000000000000000000000000000000000000000000000000000000".to_string() }
    ];

}


impl Block {
    pub fn new(utxo: Utxo) -> Block {

        let len = CHAIN.len();
        let height = len as u128 + 1;
        let prehash:String = if len>0 {CHAIN[len-1].hash.to_string()}else{"0000000000000000000000000000000000000000000000000000000000000000".into()};
        // let prehash = "".into();
        let time: u64 = if len <= 0 { 0 } else { crate::util::func::timestamp() };
        let transaction = utxo;
        let str = format!("{:?}{:?}{:?}{:?}", time.to_string(), height.to_string(), transaction.to_string(), prehash);
        let hash = crate::util::func::sha256(&str);

        Block {
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

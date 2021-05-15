use crate::util::block::Block;
use crate::util::utxo::Utxo;
use crate::util::tx::Tx;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chain {
    pub blocks: Vec<Block>
}

impl Chain {
    fn genesis() -> Block {
        Block { height: 1, hash: "6a696516b663720306ceafcabf59361090736d3b763551e5ee755ab15c633665".to_string(), time: 0, transaction: Utxo { time: 0, input: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), amount: 0.0, time: 0 }, output: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".into(), amount: 0.0, time: 0 } }, prehash: "0000000000000000000000000000000000000000000000000000000000000000".to_string() }
    }

    pub fn new() -> Self {
        Chain {
            blocks: vec![Chain::genesis()]
        }
    }
    pub fn add_block(&mut self,utxo:Utxo){
        let last_block = Self::last_block( self);
        let block = Block::new(utxo,&last_block.prehash,last_block.height);
        self.blocks.push(block);
    }

    pub fn last_block(&mut self)->&Block{
        &self.blocks[self.blocks.len()-1]
    }
}
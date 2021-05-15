pub mod util;


#[cfg(test)]
mod tests {
    use serde::de::Unexpected::Char;

    #[test]
    fn test1() {
        use crate::util::tx::Tx;
        use crate::util::utxo::Utxo;
        use crate::util::block::Block;
        use crate::util::chain::Chain;
        use crate::util::func;
        use std::fs::File;
        use std::io::Write;

        // let bk = Block::read_block("../data/1.block");
        // println!("genesis:{:?}",bk);
        let tx1 = Tx::new("addr1".into(),"token1".into(),100 as f32);
        let tx2 = tx1.clone();
        // println!("{:?}",tx1);
        let _utxo = Utxo::new(tx1,tx2);
        let _utxo1 = _utxo.clone();
        let mut chain = Chain::new();
        Chain::add_block(&mut chain,_utxo);
        Chain::add_block(&mut chain,_utxo1);
        println!("chain:{:?}",chain);
        println!("chain len:{:?}",chain.blocks.len());

        // let bk3 = Block::new(_utxo.clone(),vb);
        // println!("block3:{:?}",bk3);

        let pk = func::private_key();
        println!("pk:{:?}",pk);
        println!("address:{:?}",func::address(&pk));

        // let _json_str = block.to_string();
        // println!("{:?}",_json_str);
        // let mut file = std::fs::File::create("../data/1.block").expect("create failed");
        // file.write_all(_json_str.as_bytes()).expect("write failed");
        // assert_eq!(_block1,block);
    }
}

pub mod util;


#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        use crate::util::tx::Tx;
        use crate::util::utxo::Utxo;
        use crate::util::block::Block;
        use std::fs::File;
        use std::io::Write;
        use crate::util::block::CHAIN;

        let bk = Block::read_block("../data/1.block");

        assert_eq!(CHAIN[0],bk);

        let tx1 = Tx::new("addr1".into(),"token1".into(),100 as f32);
        let tx2 = tx1.clone();
        // println!("{:?}",tx1);
        let _utxo = Utxo::new(tx1,tx2);
        // println!("{:?}",_utxo);
        // let gddr = "00000000000000000000000000000000";
        // let gtoken = "0000000000000000000000000000000000000000000000000000000000000000";
        // let prehash = "0000000000000000000000000000000000000000000000000000000000000000";
        // println!("{}",gddr.len());
        // let _utxo1 = Utxo { time: 0, input: Tx { address: gddr.into(), token: gtoken.into(), amount: 0.0, time: 0 }, output: Tx { address: gddr.into(), token: gtoken.into(), amount: 0.0, time: 0 } };
        let block = Block::new(_utxo);
        // let _block1 = Block { height: 1, hash: "64ce755d79ca434eafe9cf4ae8c6dd456ed02577eab12a6b7a3ed79aeaed501e".into(), time: 1620911973004, transaction: Utxo { time: 1620910556591, input: Tx { address: "addr1".into(), token: "token1".into(), amount: 100.0, time: 1620910556591 }, output: Tx { address: "addr1".into(), token: "token1".into(), amount: 100.0, time: 1620910556591 } }, prehash: prehash.into() };
        println!("block:{:?}",block);
        // let _json_str = block.to_string();
        // println!("{:?}",_json_str);
        // let mut file = std::fs::File::create("../data/1.block").expect("create failed");
        // file.write_all(_json_str.as_bytes()).expect("write failed");
        // assert_eq!(_block1,block);
    }
}

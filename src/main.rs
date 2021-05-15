
use core::util::{tx,utxo};

fn main() {
    println!("Hello, world!");
    tx::test();
    let tx1 = tx::Tx::new("addr1".into(),"token1".into(),100.0);
    println!("{:?}",tx1);
    utxo::test();

}







use chrono::prelude::*;
use sha256::{digest,digest_bytes,digest_file};
use std::path::Path;

pub fn timestamp()->u64{
    let dt = Local::now();
    dt.timestamp_millis() as u64
}

pub fn sha256(input:&str)->String{
    digest(input)
}

pub fn sha256_byte(input:&[u8])->String{
    digest_bytes(input)
}

pub fn sha256_file(input:&str)->String{
    let path = Path::new(input);
    digest_file(path).unwrap()
}

pub fn private_key()->String{
    let str = sha256(&timestamp().to_string());
    str
}

pub fn address(){

}


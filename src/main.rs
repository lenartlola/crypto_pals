use std::{env, process};
use crate::utils::converters;
mod utils;

fn print_usage(pname: &str) {
    println!("Usage:");
    println!("{}: arguments needed", pname);
    process::exit(1);
}

fn main() {
    let cargs: Vec<String> = env::args().collect();
    if cargs.len() < 2 {
        print_usage(&cargs[0]);
    }
    let res = converters::hex_to_b64(&cargs[1]);
    match res {
        Ok(m) => println!("base64: {m}"),
        Err(e) => println!("Error happened: {e}")
    }
}

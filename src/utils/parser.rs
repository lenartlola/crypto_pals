use std::{env, process};

fn print_usage(pname: &str) {
    println!("Usage:");
    println!("{}: arguments needed", pname);
    process::exit(1);
}

pub fn parse_args() -> String {
    let cargs: Vec<String> = env::args().collect();
    if cargs.len() < 2 {
        print_usage(&cargs[0]);
    }
    return cargs[1].clone();
}

use std::{env, process};

use crate::{converters, xor};

fn print_usage(pname: &str) {
    println!("Usage:");
    println!();
    println!("{}: <options> |<arg1> <arg2> ... |", pname);
    println!("Options: -hb64 -fxor");
    println!("-hb64 <arg1>: convert hex to base64");
    println!("-fxor <arg1> <arg2>: perform xor on two equal length strings");
    process::exit(1);
}

pub fn parse_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
    }
    // TODO parse args and init a struct
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-hb64" => {
                if args.len() != 3 {
                    print_usage(&args[0]);
                }
                let res = converters::hex_to_b64(&args[2]);
                println!("hex to base64 : {:#?}", res.unwrap());
                process::exit(0);
            },
            "-fxor" => {
                if args.len() != 4 {
                    print_usage(&args[0]);
                }
                let res = xor::fixed_xor(&args[2], &args[3]);
                println!("fixed xor : {:#?}", res);
                process::exit(0);
            }
            _ => print_usage(&args[0])
        }
    }
}

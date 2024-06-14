use crate::utils::{converters, parser};
mod utils;


fn main() {
    let s = parser::parse_args();
    let res = converters::hex_to_b64(&s);
    match res {
        Ok(m) => println!("base64: {m}"),
        Err(e) => println!("Error happened: {e}")
    }
}

use std::collections::HashMap;
use hex;

#[allow(unused)]
pub fn fixed_xor(s1: &String, s2: &String) -> String {
    let s1_hex = hex::decode(s1).unwrap();
    let s2_hex = hex::decode(s2).unwrap();
    let mut res = String::new();

    for i in 0..s1_hex.len() {
        let xored = s1_hex[i] ^ s2_hex[i];
        res.push(xored as char);
    }
    hex::encode(res)
}

pub fn single_byte_xor(s: String) -> HashMap<String, char> {
    let s_hex = hex::decode(s).unwrap();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut ret: HashMap<String, char> = HashMap::new();

    for c in chars.chars() {
        //TODO we should probably create a dictionary and test for word frequency.
        let xored: String = s_hex.iter().map(|x| (x ^ c as u8) as char).collect(); 
        ret.insert(xored, c);
    }
    ret
}

#[allow(unused)]
pub fn repeating_xor(s: String, key: String) -> String {
    let range = key.len();
    let mut c_index = 0;
    let mut ret = String::new();

    for c in s.chars() {
        ret.push((c as u8 ^ key.as_bytes()[c_index]) as char);
        c_index += 1;
        if c_index == range {
            c_index = 0;
        }
    }
    hex::encode(ret)
}

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

pub fn single_byte_xor(s: String) -> Vec<String> {
    let s_hex = hex::decode(s).unwrap();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut ret: Vec<String> = Vec::new();

    for c in chars.chars() {
        //TODO we should probably create a dictionary and test for word frequency.
        let xored: String = s_hex.iter().map(|x| (x ^ c as u8) as char).collect(); 
        ret.push(xored);
    }
    ret
}


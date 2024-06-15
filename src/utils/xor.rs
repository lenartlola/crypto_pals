use hex;

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

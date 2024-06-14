use hex;
use base64::{Engine as _, engine::general_purpose};

pub fn hex_to_b64(s: &String) -> Result<String, String> {
    let hex_str = hex::decode(&s);
    match hex_str {
        Ok(n) => {
            let encoded_b64 = general_purpose::STANDARD.encode(n);
            return Ok(encoded_b64);
        },
        Err(e) => return Err(e.to_string()), //println!("Error happened: {:?}", e)
    }
}

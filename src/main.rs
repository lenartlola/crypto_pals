use crate::utils::xor;
mod utils;

fn main() {
    let sbx = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("Hello crypto pals!");
    let single_xored = xor::single_byte_xor(sbx.to_string());
    println!("Possible value for single byte xored:");
    println!("{:#?}", single_xored);
}

#[cfg(test)]
mod tests {
    use crate::utils::{converters, xor};
    #[test]
    fn test_h2b64() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result = converters::hex_to_b64(&s.to_string());
        assert_eq!(expected.to_string(), result.unwrap());
    }

    #[test]
    fn test_fxor() {
        let s1 = "1c0111001f010100061a024b53535009181c";
        let s2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let result = xor::fixed_xor(&s1.to_string(), &s2.to_string());
        assert_eq!(expected.to_string(), result);
    }

    #[test]
    fn test_repeating_xor() {
        let s1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";

        let res1 = xor::repeating_xor(s1.to_string(), key.to_string());
        let expected1 = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(expected1.to_string(), res1);
    }
}

pub struct Base2;

use super::utils::*;
use super::Base;

impl Base for Base2 {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        let n = decoded.to_integer();
        let encoded = to_base(&n, &"01".to_string(), Some(8));
        Ok(encoded)
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base2 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base = Base2;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001"
        );
    }

    #[test]
    fn test_decode() {
        let base = Base2;
        assert_eq!(
            base.decode(&String::from("010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001"))
                .unwrap(),
            "Hello World!"
        );
    }
}

pub struct Base2;

use super::utils::*;
use super::Base;

impl Base for Base2 {
    fn get_name(&self) -> &'static str {
        "base2"
    }
    fn get_short_name(&self) -> &'static str {
        "b2"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let n = str_to_int(decoded);
        let encoded = to_base(&n, &"01".to_string(), 8);
        Ok(encoded)
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let n = from_base(&encoded, &"01".to_string())?;
        let decoded = int_to_str(&n);
        Ok(decoded.to_string())
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

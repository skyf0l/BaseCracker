pub struct Hex;

use super::Base;

impl Base for Hex {
    fn get_name(&self) -> &'static str {
        "hex"
    }
    fn get_short_name(&self) -> &'static str {
        "h"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut encoded = String::new();
        for c in decoded.chars() {
            encoded.push_str(&format!("{:02x}", c as u8));
        }
        Ok(encoded)
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut decoded = String::new();
        for i in (0..encoded.len()).step_by(2) {
            let c = u8::from_str_radix(&encoded[i..i + 2], 16)?;
            decoded.push(c as char);
        }
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base = Hex;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "48656c6c6f20576f726c6421"
        );
    }

    #[test]
    fn test_decode() {
        let base = Hex;
        assert_eq!(
            base.decode(&String::from("48656c6c6f20576f726c6421"))
                .unwrap(),
            "Hello World!"
        );
    }
}

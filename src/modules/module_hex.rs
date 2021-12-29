pub struct Hex;

use super::*;

impl Base for Hex {
    fn get_name(&self) -> &'static str {
        "hex"
    }
    fn get_short_name(&self) -> &'static str {
        "h"
    }
    fn get_base(&self) -> &'static str {
        "0123456789abcdef"
    }
    fn get_padding(&self) -> Option<&'static str> {
        None
    }
    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 2)
    }
    fn decode(&self, encoded: &str) -> Result<String, String> {
        decode_decimal(encoded, self.get_base())
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

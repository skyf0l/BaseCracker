pub struct Hex;
use super::Base;

impl Base for Hex {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        let mut encoded = String::new();
        for c in decoded.chars() {
            encoded.push_str(&format!("{:02x}", c as u8));
        }
        Ok(encoded)
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
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
    use super::super::Base;
    use super::Hex;

    #[test]
    fn test_encode() {
        let hex = Hex;
        assert_eq!(
            hex.encode(&String::from("Hello World!")).unwrap(),
            "48656c6c6f20576f726c6421"
        );
    }

    #[test]
    fn test_decode() {
        let hex = Hex;
        assert_eq!(
            hex.decode(&String::from("48656c6c6f20576f726c6421"))
                .unwrap(),
            "Hello World!"
        );
    }
}

pub struct Base32;

use super::Base;

impl Base for Base32 {
    fn get_name(&self) -> &'static str {
        "base32"
    }
    fn get_short_name(&self) -> &'static str {
        "b32"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

pub struct Base64;

use super::Base;

impl Base for Base64 {
    fn get_name(&self) -> &'static str {
        "base64"
    }
    fn get_short_name(&self) -> &'static str {
        "b64"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

pub struct Base32;

impl super::Base for Base32 {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

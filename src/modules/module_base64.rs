pub struct Base64;

impl super::Base for Base64 {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

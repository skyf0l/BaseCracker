pub struct Hex;

impl super::Base for Hex {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Hex encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Hex decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

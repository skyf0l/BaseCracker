pub struct Base2;

impl super::Base for Base2 {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base2 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base2 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

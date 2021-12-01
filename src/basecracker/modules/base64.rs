pub struct Base64;

impl super::Base for Base64 {
    fn encode(&self, plain: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 encode: {}", plain);
        Ok(plain.to_string())
    }
    fn decode(&self, cipher: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 decode: {}", cipher);
        Ok(cipher.to_string())
    }
}

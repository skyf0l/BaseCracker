pub struct Base32;

impl super::Base for Base32 {
    fn encode(&self, plain: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 encode: {}", plain);
        Ok(plain.to_string())
    }
    fn decode(&self, cipher: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 decode: {}", cipher);
        Ok(cipher.to_string())
    }
}

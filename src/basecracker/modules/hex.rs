pub struct Hex;

impl super::Base for Hex {
    fn encode(&self, plain: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Hex encode: {}", plain);
        Ok(plain.to_string())
    }
    fn decode(&self, cipher: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Hex decode: {}", cipher);
        Ok(cipher.to_string())
    }
}

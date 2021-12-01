pub mod base32;
pub mod base64;
pub mod hex;

pub trait Base {
    fn encode(&self, plain: &String) -> Result<String, Box<dyn std::error::Error>>;
    fn decode(&self, cipher: &String) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn get_bases() -> Vec<Box<dyn Base>> {
    let bases: Vec<Box<dyn Base>> = vec![
        Box::new(hex::Hex),
        Box::new(base32::Base32),
        Box::new(base64::Base64),
    ];
    bases
}

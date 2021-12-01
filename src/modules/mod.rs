pub mod module_base32;
pub mod module_base64;
pub mod module_hex;

pub trait Base {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>>;
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn get_bases() -> Vec<Box<dyn Base>> {
    let bases: Vec<Box<dyn Base>> = vec![
        Box::new(module_hex::Hex),
        Box::new(module_base32::Base32),
        Box::new(module_base64::Base64),
    ];
    bases
}

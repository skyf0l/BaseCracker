pub mod module_base2;
pub mod module_base32;
pub mod module_base64;
pub mod module_hex;

mod utils;

pub trait Base {
    fn get_name(&self) -> &'static str;
    fn get_short_name(&self) -> &'static str;
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn get_bases() -> Vec<Box<dyn Base>> {
    let bases: Vec<Box<dyn Base>> = vec![
        Box::new(module_base2::Base2),
        Box::new(module_hex::Hex),
        Box::new(module_base32::Base32),
        Box::new(module_base64::Base64),
    ];
    bases
}
pub mod module_base10;
pub mod module_base2;
pub mod module_base32;
pub mod module_base58;
pub mod module_base62;
pub mod module_base64;
pub mod module_hex;

mod utils;
use utils::*;

pub trait Base {
    fn get_name(&self) -> &'static str;

    fn get_short_name(&self) -> &'static str;

    fn get_base(&self) -> &'static str;

    fn get_padding(&self) -> Option<&'static str>;

    fn can_be_decoded(&self, encoded: &str) -> bool {
        let mut is_padding = false;

        for c in encoded.chars() {
            if is_padding {
                match self.get_padding() {
                    Some(padding) => {
                        if !padding.contains(c) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            } else {
                if !self.get_base().contains(c) {
                    match self.get_padding() {
                        Some(padding) => {
                            if padding.contains(c) {
                                is_padding = true;
                            } else {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn encode(&self, decoded: &str) -> Result<String, String>;

    fn decode(&self, encoded: &str) -> Result<String, String>;
}

pub fn get_bases() -> Vec<Box<dyn Base>> {
    let bases: Vec<Box<dyn Base>> = vec![
        Box::new(module_base2::Base2),
        Box::new(module_base10::Base10),
        Box::new(module_hex::Hex),
        Box::new(module_base32::Base32),
        Box::new(module_base58::Base58),
        Box::new(module_base62::Base62),
        Box::new(module_base64::Base64),
    ];
    bases
}

pub fn encode_decimal(decoded: &str, base: &str, block_size: usize) -> Result<String, String> {
    let n = utils::str_to_int(decoded);
    let encoded = utils::to_base(&n, base, block_size);
    Ok(encoded)
}

pub fn decode_decimal(encoded: &str, base: &str) -> Result<String, String> {
    let n = utils::from_base(&encoded, base)?;
    let decoded = utils::int_to_str(&n);
    Ok(decoded.to_string())
}

pub fn encode_abstract(
    decoded: &str,
    base: &str,
    padding: &str,
    in_block_size: usize,
    out_block_size: usize,
) -> Result<String, String> {
    let encoded_base2 = module_base2::Base2.encode(decoded)?;
    let chunks = encoded_base2.as_str().packed_by(in_block_size);
    let mut encoded = String::new();

    for chunk in chunks {
        let chunk_value = from_base(chunk.as_str(), "01")?.to_usize().unwrap();

        if chunk.len() == in_block_size {
            encoded.push(base.chars().nth(chunk_value).unwrap());
        } else if chunk.len() < in_block_size {
            let chunk_value = chunk_value << (in_block_size - chunk.len());
            encoded.push(base.chars().nth(chunk_value).unwrap());

            let padding_len = out_block_size - encoded.len() % out_block_size;
            for _ in 0..padding_len {
                encoded.push_str(padding);
            }
        }
    }
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{module_base2::Base2, module_base64::Base64};

    #[test]
    fn test_can_be_decoded_0() {
        let base = Base2;
        assert!(base.can_be_decoded(&String::from("01001000")));
    }

    #[test]
    fn test_can_be_decoded_1() {
        let base = Base2;
        assert!(!base.can_be_decoded(&String::from("01001002")));
    }

    #[test]
    fn test_can_be_decoded_2() {
        let base = Base64;
        assert!(base.can_be_decoded(&String::from("aGVsbG8gd29ybGQ=")));
    }

    #[test]
    fn test_can_be_decoded_3() {
        let base = Base64;
        assert!(base.can_be_decoded(&String::from("YWJj")));
    }

    #[test]
    fn test_can_be_decoded_4() {
        let base = Base64;
        assert!(!base.can_be_decoded(&String::from("aGVsbG8gd29ybGQ=a")));
    }

    #[test]
    fn test_can_be_decoded_5() {
        let base = Base64;
        assert!(!base.can_be_decoded(&String::from("aGVsbG8gd29!ybGQ=")));
    }

    #[test]
    fn test_can_be_decoded_6() {
        let base = Base64;
        assert!(!base.can_be_decoded(&String::from("YW%Jj")));
    }
}

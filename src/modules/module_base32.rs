pub struct Base32;

use super::module_base2::Base2;
use super::utils::*;
use super::Base;

const BASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const PADDING: &str = "=";

impl Base for Base32 {
    fn get_name(&self) -> &'static str {
        "base32"
    }
    fn get_short_name(&self) -> &'static str {
        "b32"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encoded_base2 = Base2.encode(decoded)?;
        let chunks = encoded_base2.as_str().packed_by(5);
        let mut encoded = String::new();

        for chunk in chunks {
            let chunk_value = from_base(chunk.as_str(), "01")?.to_usize().unwrap();

            if chunk.len() == 5 {
                encoded.push(BASE.chars().nth(chunk_value).unwrap());
            } else if chunk.len() < 5 {
                let chunk_value = chunk_value << (5 - chunk.len());
                encoded.push(BASE.chars().nth(chunk_value).unwrap());

                let padding = 8 - encoded.len() % 8;
                for _ in 0..padding {
                    encoded.push_str(PADDING);
                }
            }
        }
        Ok(encoded)
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("base32 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_encode_1() {
        let result = Base32.encode("hello world").unwrap();
        assert_eq!(result, "NBSWY3DPEB3W64TMMQ======");
    }

    #[test]
    fn test_base32_encode_2() {
        let result = Base32.encode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base32_encode_3() {
        let result = Base32.encode("a").unwrap();
        assert_eq!(result, "ME======");
    }

    #[test]
    fn test_base32_encode_4() {
        let result = Base32.encode("ab").unwrap();
        assert_eq!(result, "MFRA====");
    }

    #[test]
    fn test_base32_encode_5() {
        let result = Base32.encode("abc").unwrap();
        assert_eq!(result, "MFRGG===");
    }

    #[test]
    fn test_base32_encode_6() {
        let result = Base32.encode("abcd").unwrap();
        assert_eq!(result, "MFRGGZA=");
    }

    #[test]
    fn test_base32_encode_7() {
        let result = Base32.encode("abcde").unwrap();
        assert_eq!(result, "MFRGGZDF");
    }

    #[test]
    fn test_base32_encode_8() {
        let result = Base32.encode("abcdef").unwrap();
        assert_eq!(result, "MFRGGZDFMY======");
    }
}

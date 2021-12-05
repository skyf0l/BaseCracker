pub struct Base64;

use super::module_base2::Base2;
use super::utils::*;
use super::*;

const BASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING: &str = "=";

impl Base for Base64 {
    fn get_name(&self) -> &'static str {
        "base64"
    }
    fn get_short_name(&self) -> &'static str {
        "b64"
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        encode_abstract(decoded, BASE, PADDING, 6, 4)
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut decoded_base2 = String::new();
        for c in encoded.chars() {
            if BASE.contains(c) {
                let index = BASE.find(c).unwrap();
                if index == 0 {
                    decoded_base2.push_str("000000");
                } else {
                    decoded_base2.push_str(&to_base(&Integer::from(index), "01", 6));
                }
            } else if PADDING.contains(c) {
                if decoded_base2.len() < 2 {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid padding",
                    )));
                }
                decoded_base2.pop();
                decoded_base2.pop();
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid character in base64 string",
                )));
            }
        }
        Base2.decode(decoded_base2.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_1() {
        let base64 = Base64;
        let result = base64.encode("hello world").unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_encode_2() {
        let base64 = Base64;
        let result = base64.encode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base64_encode_3() {
        let base64 = Base64;
        let result = base64.encode("a").unwrap();
        assert_eq!(result, "YQ==");
    }

    #[test]
    fn test_base64_encode_4() {
        let base64 = Base64;
        let result = base64.encode("ab").unwrap();
        assert_eq!(result, "YWI=");
    }

    #[test]
    fn test_base64_encode_5() {
        let base64 = Base64;
        let result = base64.encode("abc").unwrap();
        assert_eq!(result, "YWJj");
    }

    #[test]
    fn test_base64_encode_6() {
        let base64 = Base64;
        let result = base64.encode("abca").unwrap();
        assert_eq!(result, "YWJjYQ==");
    }

    #[test]
    fn test_base64_decode_1() {
        let base64 = Base64;
        let result = base64.decode("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_base64_decode_2() {
        let base64 = Base64;
        let result = base64.decode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base64_decode_3() {
        let base64 = Base64;
        let result = base64.decode("YQ==").unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_base64_decode_4() {
        let base64 = Base64;
        let result = base64.decode("YWI=").unwrap();
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_base64_decode_5() {
        let base64 = Base64;
        let result = base64.decode("YWJj").unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_base64_decode_6() {
        let base64 = Base64;
        let result = base64.decode("YWJjYQ==").unwrap();
        assert_eq!(result, "abca");
    }
}

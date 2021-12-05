pub struct Base32;

use super::module_base2::Base2;
use super::*;

/*
01100001 ME======
01100 00100 00000 00000 00000 00000 00000 00000
6 -> 32

01100001 01100010 MFRA====
01100 00101 10001 00000 00000 00000 00000 00000
4 -> 24

01100001 01100010 01100011 MFRGG===
01100 00101 10001 00110 00110 00000 00000 00000
3 -> 16

01100001 01100010 01100011 01100100 MFRGGZA=
01100 00101 10001 00110 00110 11001 00000 00000
1 -> 8

01100001 01100010 01100011 01100100 01100101 MFRGGZDF
01100 00101 10001 00110 00110 11001 00011 00101
*/

impl Base for Base32 {
    fn get_name(&self) -> &'static str {
        "base32"
    }
    fn get_short_name(&self) -> &'static str {
        "b32"
    }
    fn get_base(&self) -> &'static str {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
    }
    fn get_padding(&self) -> Option<&'static str> {
        Some("=")
    }
    fn encode(&self, decoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };

        encode_abstract(decoded, self.get_base(), padding, 5, 8)
    }
    fn decode(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };
        let mut decoded_base2 = String::new();
        let mut nb_padding = 0;
        for c in encoded.chars() {
            if self.get_base().contains(c) {
                let index = self.get_base().find(c).unwrap();
                if index == 0 {
                    decoded_base2.push_str("00000");
                } else {
                    decoded_base2.push_str(&to_base(&Integer::from(index), "01", 5));
                }
            } else if padding.contains(c) {
                nb_padding += 1;
                decoded_base2.push_str("00000");
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid character in base32 string",
                )));
            }
        }
        let decoded = Base2.decode(decoded_base2.as_str())?;
        match nb_padding {
            0 => Ok(decoded),
            1 => Ok(decoded.chars().take(decoded.len() - 1).collect()),
            3 => Ok(decoded.chars().take(decoded.len() - 2).collect()),
            4 => Ok(decoded.chars().take(decoded.len() - 3).collect()),
            6 => Ok(decoded.chars().take(decoded.len() - 4).collect()),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid padding",
            ))),
        }
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

    #[test]
    fn test_base32_decode_1() {
        let result = Base32.decode("NBSWY3DPEB3W64TMMQ======").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_base32_decode_2() {
        let result = Base32.decode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base32_decode_3() {
        let result = Base32.decode("ME======").unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_base32_decode_4() {
        let result = Base32.decode("MFRA====").unwrap();
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_base32_decode_5() {
        let result = Base32.decode("MFRGG===").unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_base32_decode_6() {
        let result = Base32.decode("MFRGGZA=").unwrap();
        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_base32_decode_7() {
        let result = Base32.decode("MFRGGZDF").unwrap();
        assert_eq!(result, "abcde");
    }

    #[test]
    fn test_base32_decode_8() {
        let result = Base32.decode("MFRGGZDFMY======").unwrap();
        assert_eq!(result, "abcdef");
    }
}

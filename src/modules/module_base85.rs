pub struct Base85;

use super::module_base2::Base2;
use super::utils::*;
use super::*;

impl Base for Base85 {
    fn get_name(&self) -> &'static str {
        "base85"
    }
    fn get_short_name(&self) -> &'static str {
        "b85"
    }
    fn get_base(&self) -> &'static str {
        "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu"
    }
    fn get_padding(&self) -> Option<&'static str> {
        None
    }
    fn encode(&self, decoded: &str) -> Result<String, String> {
        let mut encoded = String::new();

        // calculate padding length
        let padding = if decoded.len() % 4 != 0 {
            4 - decoded.len() % 4
        } else {
            0
        };
        // add padding
        let decoded = decoded.to_string() + &"\0".repeat(padding);

        // split the decoded string into chunks of 4 bytes
        for chunk in decoded.as_str().packed_by(4) {
            let mut value = 0;
            for c in chunk.chars() {
                value *= 256;
                value += c as usize;
            }
            encoded.push(self.get_base().chars().nth(value / 52200625).unwrap());
            let remainder = value % 52200625;
            encoded.push(self.get_base().chars().nth(remainder / 614125).unwrap());
            let remainder = remainder % 614125;
            encoded.push(self.get_base().chars().nth(remainder / 7225).unwrap());
            let remainder = remainder % 7225;
            encoded.push(self.get_base().chars().nth(remainder / 85).unwrap());
            encoded.push(self.get_base().chars().nth(remainder % 85).unwrap());
        }

        // remove padding
        for _ in 0..padding {
            encoded.pop();
        }
        Ok(encoded)
    }
    fn decode(&self, encoded: &str) -> Result<String, String> {
        let mut decoded = String::new();

        // calculate padding length
        let padding = if encoded.len() % 5 != 0 {
            5 - encoded.len() % 5
        } else {
            0
        };
        // add padding
        let encoded = encoded.to_string() + &"u".repeat(padding);

        // split the encoded string into chunks of 5 bytes
        for chunk in encoded.as_str().packed_by(5) {
            let mut value = 0;
            for c in chunk.chars() {
                value *= 85;
                value += match self.get_base().chars().position(|x| x == c) {
                    Some(i) => i,
                    None => return Err(format!("Invalid character {}", c)),
                };
            }
            decoded.push((value >> 24) as u8 as char);
            decoded.push(((value >> 16) & 255) as u8 as char);
            decoded.push(((value >> 8) & 255) as u8 as char);
            decoded.push((value & 255) as u8 as char);
        }

        // remove padding
        for _ in 0..padding {
            decoded.pop();
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base85_encode_1() {
        let base85 = Base85;
        let result = base85.encode("Hello World!").unwrap();
        assert_eq!(result, "87cURD]i,\"Ebo80");
    }

    #[test]
    fn test_base85_encode_2() {
        let base85 = Base85;
        let result = base85.encode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base85_encode_3() {
        let base85 = Base85;
        let result = base85.encode("a").unwrap();
        assert_eq!(result, "@/");
    }

    #[test]
    fn test_base85_encode_4() {
        let base85 = Base85;
        let result = base85.encode("ab").unwrap();
        assert_eq!(result, "@:B");
    }

    #[test]
    fn test_base85_encode_5() {
        let base85 = Base85;
        let result = base85.encode("abc").unwrap();
        assert_eq!(result, "@:E^");
    }

    #[test]
    fn test_base85_encode_6() {
        let base85 = Base85;
        let result = base85.encode("abca").unwrap();
        assert_eq!(result, "@:E_T");
    }

    #[test]
    fn test_base85_decode_1() {
        let base85 = Base85;
        let result = base85.decode("87cURD]i,\"Ebo80").unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_base85_decode_2() {
        let base85 = Base85;
        let result = base85.decode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base85_decode_3() {
        let base85 = Base85;
        let result = base85.decode("@/").unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_base85_decode_4() {
        let base85 = Base85;
        let result = base85.decode("@:B").unwrap();
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_base85_decode_5() {
        let base85 = Base85;
        let result = base85.decode("@:E^").unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_base85_decode_6() {
        let base85 = Base85;
        let result = base85.decode("@:E_T").unwrap();
        assert_eq!(result, "abca");
    }
}

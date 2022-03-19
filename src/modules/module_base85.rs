pub struct Base85;

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
#[cfg(not(tarpaulin_include))]
#[test]
fn test_encode_decode() {
    let base = Base85;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "87cURD]i,\"Ebo80"),
        ("BaseCracker", "6=FqH6ZQUFCLqM"),
        ("\x7fELF", "Imm%#"),
        ("", ""),
        ("a", "@/"),
        ("aa", "@:9"),
        ("aaa", "@:<R"),
        ("aaaa", "@:<SQ"),
        ("aaaaa", "@:<SQ@/"),
        ("aaaaaa", "@:<SQ@:9"),
    ];

    for test in TESTLIST.iter() {
        // encode
        let encoded = match base.encode(&test.0) {
            Ok(encoded) => encoded,
            Err(e) => panic!("Error while encoding \"{}\": {}", test.0, e),
        };
        assert_eq!(encoded, test.1, "Encoding \"{}\" failed", test.0);

        // decode
        let decoded = match base.decode(&encoded) {
            Ok(decoded) => decoded,
            Err(e) => panic!("Error while decoding \"{}\": {}", encoded, e),
        };
        assert_eq!(decoded, test.0, "Decoding \"{}\" failed", encoded);
    }
}

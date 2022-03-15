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

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };

        encode_abstract(decoded, self.get_base(), padding, 5, 8)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };

        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
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
                return Err("Invalid character in base32 string".to_string());
            }
        }
        let decoded = Base2.decode(decoded_base2.as_str())?;
        match nb_padding {
            0 => Ok(decoded),
            1 => Ok(decoded.chars().take(decoded.len() - 1).collect()),
            3 => Ok(decoded.chars().take(decoded.len() - 2).collect()),
            4 => Ok(decoded.chars().take(decoded.len() - 3).collect()),
            6 => Ok(decoded.chars().take(decoded.len() - 4).collect()),
            _ => Err("Invalid padding".to_string()),
        }
    }
}

#[cfg(test)]
#[test]
fn test_encode_decode() {
    let base = Base32;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "JBSWY3DPEBLW64TMMQQQ===="),
        ("BaseCracker", "IJQXGZKDOJQWG23FOI======"),
        ("\x7fELF", "P5CUYRQ="),
        ("", ""),
        ("a", "ME======"),
        ("aa", "MFQQ===="),
        ("aaa", "MFQWC==="),
        ("aaaa", "MFQWCYI="),
        ("aaaaa", "MFQWCYLB"),
        ("aaaaaa", "MFQWCYLBME======"),
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

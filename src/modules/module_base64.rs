pub struct Base64;

use super::module_base2::Base2;
use super::utils::*;
use super::*;

impl Base for Base64 {
    fn get_name(&self) -> &'static str {
        "base64"
    }

    fn get_short_name(&self) -> &'static str {
        "b64"
    }

    fn get_base(&self) -> &'static str {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    }

    fn get_padding(&self) -> Option<&'static str> {
        Some("=")
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };

        encode_abstract(decoded, self.get_base(), padding, 6, 4)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let padding = match self.get_padding() {
            Some(c) => c,
            None => Err("No complement")?,
        };

        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let mut decoded_base2 = String::new();
        for c in encoded.chars() {
            if self.get_base().contains(c) {
                let index = self.get_base().find(c).unwrap();
                if index == 0 {
                    decoded_base2.push_str("000000");
                } else {
                    decoded_base2.push_str(&to_base(&Integer::from(index), "01", 6));
                }
            } else if padding.contains(c) {
                if decoded_base2.len() < 2 {
                    return Err("Invalid padding".to_string());
                }
                decoded_base2.pop();
                decoded_base2.pop();
            } else {
                return Err("Invalid character in base64 string".to_string());
            }
        }
        Base2.decode(decoded_base2.as_str())
    }
}

#[cfg(test)]
#[test]
fn test_encode_decode() {
    let base = Base64;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "SGVsbG8gV29ybGQh"),
        ("BaseCracker", "QmFzZUNyYWNrZXI="),
        ("\x7fELF", "f0VMRg=="),
        ("", ""),
        ("a", "YQ=="),
        ("aa", "YWE="),
        ("aaa", "YWFh"),
        ("aaaa", "YWFhYQ=="),
        ("aaaaa", "YWFhYWE="),
        ("aaaaaa", "YWFhYWFh"),
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

pub struct Hex;

use super::*;

impl Base for Hex {
    fn get_name(&self) -> &'static str {
        "hex"
    }

    fn get_short_name(&self) -> &'static str {
        "h"
    }

    fn get_base(&self) -> &'static str {
        "0123456789abcdef"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 2)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let encoded = encoded.to_lowercase();
        decode_decimal(encoded.as_str(), self.get_base())
    }
}

#[cfg(test)]
#[test]
fn test_encode_decode() {
    let base = Hex;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "48656c6c6f20576f726c6421"),
        ("BaseCracker", "42617365437261636b6572"),
        ("\x7fELF", "7f454c46"),
        ("", ""),
        ("a", "61"),
        ("aa", "6161"),
        ("aaa", "616161"),
        ("aaaa", "61616161"),
        ("aaaaa", "6161616161"),
        ("aaaaaa", "616161616161"),
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

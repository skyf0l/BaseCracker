pub struct Base36;

use super::*;

impl Base for Base36 {
    fn get_name(&self) -> &'static str {
        "base36"
    }

    fn get_short_name(&self) -> &'static str {
        "b32"
    }

    fn get_base(&self) -> &'static str {
        "0123456789abcdefghijklmnopqrstuvwxyz"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 1)
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
    let base = Base36;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "2678lx5gvmsv1dro9b5"),
        ("BaseCracker", "a2zww16mmx0c25wfm"),
        ("\x7fELF", "zb9ruu"),
        ("", ""),
        ("a", "2p"),
        ("aa", "j8h"),
        ("aaa", "3ssbl"),
        ("aaaa", "r0peg1"),
        ("aaaaa", "5c50mq1t"),
        ("aaaaaa", "11ybohl8wx"),
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

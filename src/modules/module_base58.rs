pub struct Base58;

use super::*;

impl Base for Base58 {
    fn get_name(&self) -> &'static str {
        "base58"
    }

    fn get_short_name(&self) -> &'static str {
        "b58"
    }

    fn get_base(&self) -> &'static str {
        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 1)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        decode_decimal(encoded.as_str(), self.get_base())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
#[test]
fn test_encode_decode() {
    let base = Base58;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "2NEpo7TZRRrLZSi2U"),
        ("BaseCracker", "HTivuUbjqtbbaC1"),
        ("\x7fELF", "4Fghph"),
        ("", ""),
        ("a", "2g"),
        ("aa", "8Qp"),
        ("aaa", "Zi88"),
        ("aaaa", "3VNWTa"),
        ("aaaaa", "BzDw2JL"),
        ("aaaaaa", "qVa5SjWY"),
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

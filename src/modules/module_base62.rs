pub struct Base62;

use super::*;

impl Base for Base62 {
    fn get_name(&self) -> &'static str {
        "base62"
    }

    fn get_short_name(&self) -> &'static str {
        "b62"
    }

    fn get_base(&self) -> &'static str {
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
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
    let base = Base62;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "T8dgcjRGkZ3aysdN"),
        ("BaseCracker", "6TBjWkfpqJ4MQks"),
        ("\x7fELF", "2KVHWw"),
        ("", ""),
        ("a", "1Z"),
        ("aa", "6U5"),
        ("aaa", "QmED"),
        ("aaaa", "1mZ8hF"),
        ("aaaaa", "7MX5uZV"),
        ("aaaaaa", "UP2ePabZ"),
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

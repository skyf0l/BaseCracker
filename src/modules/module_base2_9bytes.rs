pub struct Base2_9bytes;

use super::*;

impl Base for Base2_9bytes {
    fn get_name(&self) -> &'static str {
        "base2-9bytes"
    }

    fn get_short_name(&self) -> &'static str {
        "b2-9"
    }

    fn get_base(&self) -> &'static str {
        "01"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let n = utils::str_to_int(decoded, 512);
        let encoded = utils::to_base(&n, self.get_base(), 9);
        Ok(encoded)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let n = utils::from_base(&encoded, self.get_base())?;
        let decoded = utils::int_to_str(&n, 512);
        Ok(decoded.to_string())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
#[test]
fn test_encode_decode() {
    let base = Base2_9bytes;
    const TESTLIST: [(&str, &str); 10] = [
        (
            "Hello World!",
            "001001000001100101001101100001101100001101111000100000001010111001101111001110010001101100001100100000100001",
        ),
        (
            "BaseCracker",
            "001000010001100001001110011001100101001000011001110010001100001001100011001101011001100101001110010",
        ),
        ("\x7fELF", "001111111001000101001001100001000110"),
        ("", ""),
        ("a", "001100001"),
        ("aa", "001100001001100001"),
        ("aaa", "001100001001100001001100001"),
        ("aaaa", "001100001001100001001100001001100001"),
        ("aaaaa", "001100001001100001001100001001100001001100001"),
        ("aaaaaa", "001100001001100001001100001001100001001100001001100001"),
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

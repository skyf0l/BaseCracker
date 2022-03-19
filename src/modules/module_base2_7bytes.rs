pub struct Base2_7bytes;

use super::*;

impl Base for Base2_7bytes {
    fn get_name(&self) -> &'static str {
        "base2-7bytes"
    }

    fn get_short_name(&self) -> &'static str {
        "b2-7"
    }

    fn get_base(&self) -> &'static str {
        "01"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let n = utils::str_to_int(decoded, 128);
        let encoded = utils::to_base(&n, self.get_base(), 7);
        Ok(encoded)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let n = utils::from_base(&encoded, self.get_base())?;
        let decoded = utils::int_to_str(&n, 128);
        Ok(decoded.to_string())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
#[test]
fn test_encode_decode() {
    let base = Base2_7bytes;
    const TESTLIST: [(&str, &str); 10] = [
        (
            "Hello World!",
            "100100011001011101100110110011011110100000101011111011111110010110110011001000100001",
        ),
        (
            "BaseCracker",
            "10000101100001111001111001011000011111001011000011100011110101111001011110010",
        ),
        ("\x7fELF", "1111111100010110011001000110"),
        ("", ""),
        ("a", "1100001"),
        ("aa", "11000011100001"),
        ("aaa", "110000111000011100001"),
        ("aaaa", "1100001110000111000011100001"),
        ("aaaaa", "11000011100001110000111000011100001"),
        ("aaaaaa", "110000111000011100001110000111000011100001"),
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

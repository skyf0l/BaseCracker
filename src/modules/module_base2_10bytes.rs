pub struct Base2_10bytes;

use super::*;

impl Base for Base2_10bytes {
    fn get_name(&self) -> &'static str {
        "base2-10bytes"
    }

    fn get_short_name(&self) -> &'static str {
        "b2-10"
    }

    fn get_base(&self) -> &'static str {
        "01"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let n = utils::str_to_int(decoded, 1024);
        let encoded = utils::to_base(&n, self.get_base(), 10);
        Ok(encoded)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let n = utils::from_base(&encoded, self.get_base())?;
        let decoded = utils::int_to_str(&n, 1024);
        Ok(decoded.to_string())
    }
}

#[cfg(test)]
#[test]
fn test_encode_decode() {
    let base = Base2_10bytes;
    const TESTLIST: [(&str, &str); 10] = [
        (
            "Hello World!",
            "000100100000011001010001101100000110110000011011110000100000000101011100011011110001110010000110110000011001000000100001",
        ),
        (
            "BaseCracker",
            "00010000100001100001000111001100011001010001000011000111001000011000010001100011000110101100011001010001110010",
        ),
        ("\x7fELF", "0001111111000100010100010011000001000110"),
        ("", ""),
        ("a", "0001100001"),
        ("aa", "00011000010001100001"),
        ("aaa", "000110000100011000010001100001"),
        ("aaaa", "0001100001000110000100011000010001100001"),
        ("aaaaa", "00011000010001100001000110000100011000010001100001"),
        ("aaaaaa", "000110000100011000010001100001000110000100011000010001100001"),
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

pub struct Base2;

use super::*;

impl Base for Base2 {
    fn get_name(&self) -> &'static str {
        "base2"
    }

    fn get_short_name(&self) -> &'static str {
        "b2"
    }

    fn get_base(&self) -> &'static str {
        "01"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 8)
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
    let base = Base2;
    const TESTLIST: [(&str, &str); 10] = [
        (
            "Hello World!",
            "010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001",
        ),
        (
            "BaseCracker",
            "0100001001100001011100110110010101000011011100100110000101100011011010110110010101110010",
        ),
        ("\x7fELF", "01111111010001010100110001000110"),
        ("", ""),
        ("a", "01100001"),
        ("aa", "0110000101100001"),
        ("aaa", "011000010110000101100001"),
        ("aaaa", "01100001011000010110000101100001"),
        ("aaaaa", "0110000101100001011000010110000101100001"),
        ("aaaaaa", "011000010110000101100001011000010110000101100001"),
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

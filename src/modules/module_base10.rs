pub struct Base10;

use super::*;

impl Base for Base10 {
    fn get_name(&self) -> &'static str {
        "base10"
    }

    fn get_short_name(&self) -> &'static str {
        "b10"
    }

    fn get_base(&self) -> &'static str {
        "0123456789"
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
#[test]
fn test_encode_decode() {
    let base = Base10;
    const TESTLIST: [(&str, &str); 10] = [
        ("Hello World!", "22405534230753928650781647905"),
        ("BaseCracker", "80249302315773941590484338"),
        ("\x7fELF", "2135247942"),
        ("", ""),
        ("a", "97"),
        ("aa", "24929"),
        ("aaa", "6381921"),
        ("aaaa", "1633771873"),
        ("aaaaa", "418245599585"),
        ("aaaaaa", "107070873493857"),
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

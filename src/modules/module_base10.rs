/// Base10 module.
pub struct Base10;

use super::*;

const ALPHABET: &str = "0123456789";

impl Base for Base10 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base10",
            short_name: "b10",
            base: ALPHABET,
            padding: None,
        }
    }

    fn encode(&self, plain: &str) -> String {
        base_x::encode(ALPHABET, plain.as_bytes())
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        base_x::decode(ALPHABET, enc)
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|_| DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

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

        for (plaintext, ciphertext) in TESTLIST.iter() {
            assert_eq!(
                base.encode(plaintext),
                *ciphertext,
                "Encoding \"{plaintext}\" failed"
            );

            assert_eq!(
                base.decode(ciphertext).unwrap(),
                *plaintext,
                "Decoding \"{plaintext}\" failed"
            );
        }
    }
}

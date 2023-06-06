/// Base36 module.
pub struct Base36;

use super::*;

const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

impl Base for Base36 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base36",
            short_name: "b36",
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

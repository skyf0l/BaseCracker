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

    fn encode(&self, plain: &[u8]) -> String {
        base_x::encode(ALPHABET, plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        base_x::decode(ALPHABET, enc).map_err(|_| DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base36;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "2678lx5gvmsv1dro9b5"),
            (b"BaseCracker", "a2zww16mmx0c25wfm"),
            (b"\x7fELF", "zb9ruu"),
            (b"", ""),
            (b"a", "2p"),
            (b"aa", "j8h"),
            (b"aaa", "3ssbl"),
            (b"aaaa", "r0peg1"),
            (b"aaaaa", "5c50mq1t"),
            (b"aaaaaa", "11ybohl8wx"),
        ];

        for (plaintext, ciphertext) in TESTLIST.iter() {
            assert_eq!(
                base.encode(plaintext),
                *ciphertext,
                "Encoding \"{}\" failed",
                unsafe { std::str::from_utf8_unchecked(plaintext) }
            );

            assert_eq!(
                base.decode(ciphertext).unwrap(),
                *plaintext,
                "Decoding \"{}\" failed",
                unsafe { std::str::from_utf8_unchecked(plaintext) }
            );
        }
    }
}

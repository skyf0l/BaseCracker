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
        let base = Base10;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "22405534230753928650781647905"),
            (b"BaseCracker", "80249302315773941590484338"),
            (b"\x7fELF", "2135247942"),
            (b"", ""),
            (b"a", "97"),
            (b"aa", "24929"),
            (b"aaa", "6381921"),
            (b"aaaa", "1633771873"),
            (b"aaaaa", "418245599585"),
            (b"aaaaaa", "107070873493857"),
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

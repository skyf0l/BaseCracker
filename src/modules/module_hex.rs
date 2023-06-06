/// Hex module.
pub struct Hex;

use super::*;

impl Base for Hex {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "hex",
            short_name: "h",
            base: "0123456789abcdef",
            padding: None,
        }
    }

    fn encode(&self, plain: &[u8]) -> String {
        hex::encode(plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        hex::decode(enc).map_err(|e| match e {
            hex::FromHexError::InvalidHexCharacter { c, index } => {
                DecodeError::InvalidByte(index, c as u8)
            }
            hex::FromHexError::OddLength => DecodeError::InvalidLength,
            hex::FromHexError::InvalidStringLength => unreachable!(),
        })
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Hex;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "48656c6c6f20576f726c6421"),
            (b"BaseCracker", "42617365437261636b6572"),
            (b"\x7fELF", "7f454c46"),
            (b"", ""),
            (b"a", "61"),
            (b"aa", "6161"),
            (b"aaa", "616161"),
            (b"aaaa", "61616161"),
            (b"aaaaa", "6161616161"),
            (b"aaaaaa", "616161616161"),
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

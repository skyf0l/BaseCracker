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

    fn encode(&self, plain: &str) -> String {
        hex::encode(plain)
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        hex::decode(enc)
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|e| match e {
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

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "48656c6c6f20576f726c6421"),
            ("BaseCracker", "42617365437261636b6572"),
            ("\x7fELF", "7f454c46"),
            ("", ""),
            ("a", "61"),
            ("aa", "6161"),
            ("aaa", "616161"),
            ("aaaa", "61616161"),
            ("aaaaa", "6161616161"),
            ("aaaaaa", "616161616161"),
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

use base58::{FromBase58, ToBase58};

/// Base58 module.
pub struct Base58;

use super::*;

impl Base for Base58 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base58",
            short_name: "b58",
            base: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            padding: None,
        }
    }

    fn encode(&self, plain: &str) -> String {
        plain.as_bytes().to_base58()
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        enc.from_base58()
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|e| match e {
                base58::FromBase58Error::InvalidBase58Character(c, n) => {
                    DecodeError::InvalidByte(n, c as u8)
                }
                base58::FromBase58Error::InvalidBase58Length => DecodeError::InvalidLength,
            })
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base58;

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "2NEpo7TZRRrLZSi2U"),
            ("BaseCracker", "HTivuUbjqtbbaC1"),
            ("\x7fELF", "4Fghph"),
            ("", ""),
            ("a", "2g"),
            ("aa", "8Qp"),
            ("aaa", "Zi88"),
            ("aaaa", "3VNWTa"),
            ("aaaaa", "BzDw2JL"),
            ("aaaaaa", "qVa5SjWY"),
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

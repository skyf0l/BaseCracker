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

    fn encode(&self, plain: &[u8]) -> String {
        plain.to_base58()
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        enc.from_base58().map_err(|e| match e {
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

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "2NEpo7TZRRrLZSi2U"),
            (b"BaseCracker", "HTivuUbjqtbbaC1"),
            (b"\x7fELF", "4Fghph"),
            (b"", ""),
            (b"a", "2g"),
            (b"aa", "8Qp"),
            (b"aaa", "Zi88"),
            (b"aaaa", "3VNWTa"),
            (b"aaaaa", "BzDw2JL"),
            (b"aaaaaa", "qVa5SjWY"),
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

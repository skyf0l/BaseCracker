use base64::{engine::general_purpose, Engine};

/// Base64 module.
pub struct Base64;

use super::*;

impl Base for Base64 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base64",
            short_name: "b64",
            base: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            padding: Some("="),
        }
    }

    fn encode(&self, plain: &[u8]) -> String {
        general_purpose::STANDARD.encode(plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        general_purpose::STANDARD.decode(enc).map_err(|e| match e {
            base64::DecodeError::InvalidByte(n, c) => DecodeError::InvalidByte(n, c),
            base64::DecodeError::InvalidLength => DecodeError::InvalidLength,
            base64::DecodeError::InvalidLastSymbol(_, _) => DecodeError::Error,
            base64::DecodeError::InvalidPadding => DecodeError::InvalidPadding,
        })
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base64;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "SGVsbG8gV29ybGQh"),
            (b"BaseCracker", "QmFzZUNyYWNrZXI="),
            (b"\x7fELF", "f0VMRg=="),
            (b"", ""),
            (b"a", "YQ=="),
            (b"aa", "YWE="),
            (b"aaa", "YWFh"),
            (b"aaaa", "YWFhYQ=="),
            (b"aaaaa", "YWFhYWE="),
            (b"aaaaaa", "YWFhYWFh"),
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

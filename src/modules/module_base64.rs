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

    fn encode(&self, plain: &str) -> String {
        general_purpose::STANDARD.encode(plain)
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        general_purpose::STANDARD
            .decode(enc)
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|e| match e {
                base64::DecodeError::InvalidByte(n, c) => DecodeError::InvalidByte(n, c),
                base64::DecodeError::InvalidLength => DecodeError::InvalidLength,
                base64::DecodeError::InvalidLastSymbol(n, c) => {
                    DecodeError::InvalidLastSymbol(n, c)
                }
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

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "SGVsbG8gV29ybGQh"),
            ("BaseCracker", "QmFzZUNyYWNrZXI="),
            ("\x7fELF", "f0VMRg=="),
            ("", ""),
            ("a", "YQ=="),
            ("aa", "YWE="),
            ("aaa", "YWFh"),
            ("aaaa", "YWFhYQ=="),
            ("aaaaa", "YWFhYWE="),
            ("aaaaaa", "YWFhYWFh"),
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

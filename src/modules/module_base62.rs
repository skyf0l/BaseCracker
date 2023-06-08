/// Base62 module.
pub struct Base62;

use super::*;

impl Base for Base62 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base62",
            short_name: "b62",
            base: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            padding: None,
        }
    }

    fn encode(&self, plain: &[u8]) -> String {
        bs62::encode_data(plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        if enc.is_empty() {
            return Ok(vec![]);
        }
        bs62::decode_data_forgiving(enc).map_err(|_| DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base62;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "28B5ymDkgIUeiuVwP"),
            (b"BaseCracker", "VQOOjhqLhZROpr0"),
            (b"\x7fELF", "71AWj0"),
            (b"", "1"),
            (b"a", "5h"),
            (b"aa", "NX7"),
            (b"aaa", "1ZAkT"),
            (b"aaaa", "6TENtJ"),
            (b"aaaaa", "QihOeO1"),
            (b"aaaaaa", "1mKZBmlBh"),
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

    #[test]
    fn test_decode_forgiving() {
        let base = Base62;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "T8dgcjRGkZ3aysdN"),
            (b"BaseCracker", "6TBjWkfpqJ4MQks"),
            (b"\x7fELF", "2KVHWw"),
            (b"", ""),
            (b"a", "1Z"),
            (b"aa", "6U5"),
            (b"aaa", "QmED"),
            (b"aaaa", "1mZ8hF"),
            (b"aaaaa", "7MX5uZV"),
            (b"aaaaaa", "UP2ePabZ"),
        ];

        for (plaintext, ciphertext) in TESTLIST.iter() {
            assert_eq!(
                base.decode(ciphertext).unwrap(),
                *plaintext,
                "Decoding \"{}\" failed",
                unsafe { std::str::from_utf8_unchecked(plaintext) }
            );
        }
    }
}

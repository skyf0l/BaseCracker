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

    fn encode(&self, plain: &str) -> String {
        bs62::encode_data(plain.as_bytes())
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        if enc.is_empty() {
            return Ok(String::new());
        }
        bs62::decode_data_forgiving(enc)
            .map(|v| String::from_utf8(v).unwrap())
            .map_err(|_| DecodeError::UnknownError)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base62;

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "28B5ymDkgIUeiuVwP"),
            ("BaseCracker", "VQOOjhqLhZROpr0"),
            ("\x7fELF", "71AWj0"),
            ("", "1"),
            ("a", "5h"),
            ("aa", "NX7"),
            ("aaa", "1ZAkT"),
            ("aaaa", "6TENtJ"),
            ("aaaaa", "QihOeO1"),
            ("aaaaaa", "1mKZBmlBh"),
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

    #[test]
    fn test_decode_forgiving() {
        let base = Base62;

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "T8dgcjRGkZ3aysdN"),
            ("BaseCracker", "6TBjWkfpqJ4MQks"),
            ("\x7fELF", "2KVHWw"),
            ("", ""),
            ("a", "1Z"),
            ("aa", "6U5"),
            ("aaa", "QmED"),
            ("aaaa", "1mZ8hF"),
            ("aaaaa", "7MX5uZV"),
            ("aaaaaa", "UP2ePabZ"),
        ];

        for (plaintext, ciphertext) in TESTLIST.iter() {
            assert_eq!(
                base.decode(ciphertext).unwrap(),
                *plaintext,
                "Decoding \"{plaintext}\" failed"
            );
        }
    }
}

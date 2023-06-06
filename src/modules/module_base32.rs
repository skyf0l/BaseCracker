/// Base32 module.
pub struct Base32;

use super::*;

const ALPHABET: base32::Alphabet = base32::Alphabet::RFC4648 { padding: true };

impl Base for Base32 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base32",
            short_name: "b32",
            base: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
            padding: Some("="),
        }
    }

    fn encode(&self, plain: &str) -> String {
        base32::encode(ALPHABET, plain.as_bytes())
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        base32::decode(ALPHABET, enc)
            .map(|v| String::from_utf8(v).unwrap())
            .ok_or(DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base32;

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "JBSWY3DPEBLW64TMMQQQ===="),
            ("BaseCracker", "IJQXGZKDOJQWG23FOI======"),
            ("\x7fELF", "P5CUYRQ="),
            ("", ""),
            ("a", "ME======"),
            ("aa", "MFQQ===="),
            ("aaa", "MFQWC==="),
            ("aaaa", "MFQWCYI="),
            ("aaaaa", "MFQWCYLB"),
            ("aaaaaa", "MFQWCYLBME======"),
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

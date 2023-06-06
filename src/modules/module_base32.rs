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

    fn encode(&self, plain: &[u8]) -> String {
        base32::encode(ALPHABET, plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        base32::decode(ALPHABET, enc).ok_or(DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base32;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "JBSWY3DPEBLW64TMMQQQ===="),
            (b"BaseCracker", "IJQXGZKDOJQWG23FOI======"),
            (b"\x7fELF", "P5CUYRQ="),
            (b"", ""),
            (b"a", "ME======"),
            (b"aa", "MFQQ===="),
            (b"aaa", "MFQWC==="),
            (b"aaaa", "MFQWCYI="),
            (b"aaaaa", "MFQWCYLB"),
            (b"aaaaaa", "MFQWCYLBME======"),
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

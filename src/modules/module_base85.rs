/// Base85 module.
pub struct Base85;

use super::*;

impl Base for Base85 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base85",
            short_name: "b85",
            base: "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu",
            padding: None,
        }
    }

    fn encode(&self, plain: &[u8]) -> String {
        base85::encode(plain)
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        base85::decode(enc).ok_or(DecodeError::Error)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base85;

        const TESTLIST: [(&[u8], &str); 10] = [
            (b"Hello World!", "NM&qnZy;B1a%^NF"),
            (b"BaseCracker", "LSb`dLvmqbYh`i"),
            (b"\x7fELF", "e??42"),
            (b"", ""),
            (b"a", "VE"),
            (b"aa", "VPO"),
            (b"aaa", "VPRn"),
            (b"aaaa", "VPRom"),
            (b"aaaaa", "VPRomVE"),
            (b"aaaaaa", "VPRomVPO"),
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

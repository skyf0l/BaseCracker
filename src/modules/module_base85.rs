use base85;

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

    fn encode(&self, plain: &str) -> String {
        base85::encode(plain.as_bytes())
    }

    fn decode(&self, enc: &str) -> Result<String, DecodeError> {
        base85::decode(enc)
            .map(|v| String::from_utf8(v).unwrap())
            .ok_or(DecodeError::UnknownError)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base85;

        const TESTLIST: [(&str, &str); 10] = [
            ("Hello World!", "NM&qnZy;B1a%^NF"),
            ("BaseCracker", "LSb`dLvmqbYh`i"),
            ("\x7fELF", "e??42"),
            ("", ""),
            ("a", "VE"),
            ("aa", "VPO"),
            ("aaa", "VPRn"),
            ("aaaa", "VPRom"),
            ("aaaaa", "VPRomVE"),
            ("aaaaaa", "VPRomVPO"),
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

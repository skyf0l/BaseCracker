/// Base2 module.
pub struct Base2;

use super::*;

const ALPHABET: &str = "01";

impl Base for Base2 {
    fn get_metadata(&self) -> &'static BaseMetadata {
        &BaseMetadata {
            name: "base2",
            short_name: "b2",
            base: ALPHABET,
            padding: None,
        }
    }

    fn encode(&self, plain: &[u8]) -> String {
        plain
            .iter()
            .map(|&byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .concat()
    }

    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError> {
        enc.chars()
            .collect::<Vec<char>>()
            .chunks(8)
            .map(|chunk| {
                let byte_str: String = chunk.into_iter().collect();
                u8::from_str_radix(&byte_str, 2).map_err(|_| DecodeError::Error)
            })
            .collect::<Result<Vec<u8>, DecodeError>>()
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let base = Base2;

        const TESTLIST: [(&[u8], &str); 10] = [
            (
                b"Hello World!",
                "010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001",
            ),
            (
                b"BaseCracker",
                "0100001001100001011100110110010101000011011100100110000101100011011010110110010101110010",
            ),
            (b"\x7fELF", "01111111010001010100110001000110"),
            (b"", ""),
            (b"a", "01100001"),
            (b"aa", "0110000101100001"),
            (b"aaa", "011000010110000101100001"),
            (b"aaaa", "01100001011000010110000101100001"),
            (b"aaaaa", "0110000101100001011000010110000101100001"),
            (b"aaaaaa", "011000010110000101100001011000010110000101100001"),
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

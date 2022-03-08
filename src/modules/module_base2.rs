pub struct Base2;

use super::*;

impl Base for Base2 {
    fn get_name(&self) -> &'static str {
        "base2"
    }
    fn get_short_name(&self) -> &'static str {
        "b2"
    }
    fn get_base(&self) -> &'static str {
        "01"
    }
    fn get_padding(&self) -> Option<&'static str> {
        None
    }
    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 8)
    }
    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        decode_decimal(encoded.as_str(), self.get_base())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base = Base2;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001"
        );
    }

    #[test]
    fn test_decode() {
        let base = Base2;
        assert_eq!(
            base.decode(&String::from("010010000110010101101100011011000110111100100000010101110110111101110010011011000110010000100001"))
                .unwrap(),
            "Hello World!"
        );
    }

    #[test]
    fn test_decode_blankspaces() {
        let base = Base2;
        assert_eq!(
            base.decode(&String::from("010010  00 0 110 0 1010\t110 11 000  110110001101111  \t 00100000010101110110 \t111\t 101110\t \t010011011000110010000100001 "))
                .unwrap(),
            "Hello World!"
        );
    }
}

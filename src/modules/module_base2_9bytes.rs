pub struct Base2_9bytes;

use super::*;

impl Base for Base2_9bytes {
    fn get_name(&self) -> &'static str {
        "base2-9bytes"
    }
    fn get_short_name(&self) -> &'static str {
        "b2-9"
    }
    fn get_base(&self) -> &'static str {
        "01"
    }
    fn get_padding(&self) -> Option<&'static str> {
        None
    }
    fn encode(&self, decoded: &str) -> Result<String, String> {
        let n = utils::str_to_int(decoded, 512);
        let encoded = utils::to_base(&n, self.get_base(), 9);
        Ok(encoded)
    }
    fn decode(&self, encoded: &str) -> Result<String, String> {
        let n = utils::from_base(&encoded, self.get_base())?;
        let decoded = utils::int_to_str(&n, 512);
        Ok(decoded.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base = Base2_9bytes;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "001001000001100101001101100001101100001101111000100000001010111001101111001110010001101100001100100000100001"
        );
    }

    #[test]
    fn test_decode() {
        let base = Base2_9bytes;
        assert_eq!(
            base.decode(&String::from("001001000001100101001101100001101100001101111000100000001010111001101111001110010001101100001100100000100001"))
                .unwrap(),
            "Hello World!"
        );
    }
}

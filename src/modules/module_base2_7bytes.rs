pub struct Base2_7bytes;

use super::*;

impl Base for Base2_7bytes {
    fn get_name(&self) -> &'static str {
        "base2-7bytes"
    }

    fn get_short_name(&self) -> &'static str {
        "b2-7"
    }

    fn get_base(&self) -> &'static str {
        "01"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        let n = utils::str_to_int(decoded, 128);
        let encoded = utils::to_base(&n, self.get_base(), 7);
        Ok(encoded)
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let encoded = encoded.replace("\n", "").replace(" ", "").replace("\t", "");
        let n = utils::from_base(&encoded, self.get_base())?;
        let decoded = utils::int_to_str(&n, 128);
        Ok(decoded.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let base = Base2_7bytes;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "100100011001011101100110110011011110100000101011111011111110010110110011001000100001"
        );
    }

    #[test]
    fn test_decode() {
        let base = Base2_7bytes;
        assert_eq!(
            base.decode(&String::from("100100011001011101100110110011011110100000101011111011111110010110110011001000100001"))
                .unwrap(),
            "Hello World!"
        );
    }
}

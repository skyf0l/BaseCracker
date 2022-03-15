pub struct Base10;

use super::*;

impl Base for Base10 {
    fn get_name(&self) -> &'static str {
        "base10"
    }

    fn get_short_name(&self) -> &'static str {
        "b10"
    }

    fn get_base(&self) -> &'static str {
        "0123456789"
    }

    fn get_padding(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 1)
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
    fn test_encode_1() {
        let base = Base10;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "22405534230753928650781647905"
        );
    }

    #[test]
    fn test_encode_2() {
        let base = Base10;
        assert_eq!(base.encode(&String::from("a")).unwrap(), "97");
    }

    #[test]
    fn test_encode_3() {
        let base = Base10;
        assert_eq!(base.encode(&String::from("ab")).unwrap(), "24930");
    }

    #[test]
    fn test_encode_4() {
        let base = Base10;
        assert_eq!(base.encode(&String::from("abc")).unwrap(), "6382179");
    }

    #[test]
    fn test_decode_1() {
        let base = Base10;
        assert_eq!(
            base.decode(&String::from("22405534230753928650781647905"))
                .unwrap(),
            "Hello World!"
        );
    }

    #[test]
    fn test_decode_2() {
        let base = Base10;
        assert_eq!(base.decode(&String::from("97")).unwrap(), "a");
    }

    #[test]
    fn test_decode_3() {
        let base = Base10;
        assert_eq!(base.decode(&String::from("24930")).unwrap(), "ab");
    }

    #[test]
    fn test_decode_4() {
        let base = Base10;
        assert_eq!(base.decode(&String::from("6382179")).unwrap(), "abc");
    }
}

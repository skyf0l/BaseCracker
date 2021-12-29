pub struct Base58;

use super::*;

impl Base for Base58 {
    fn get_name(&self) -> &'static str {
        "base58"
    }
    fn get_short_name(&self) -> &'static str {
        "b58"
    }
    fn get_base(&self) -> &'static str {
        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    }
    fn get_padding(&self) -> Option<&'static str> {
        None
    }
    fn encode(&self, decoded: &str) -> Result<String, String> {
        encode_decimal(decoded, self.get_base(), 1)
    }
    fn decode(&self, encoded: &str) -> Result<String, String> {
        decode_decimal(encoded, self.get_base())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_1() {
        let base = Base58;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "2NEpo7TZRRrLZSi2U"
        );
    }

    #[test]
    fn test_encode_2() {
        let base = Base58;
        assert_eq!(base.encode(&String::from("a")).unwrap(), "2g");
    }

    #[test]
    fn test_encode_3() {
        let base = Base58;
        assert_eq!(base.encode(&String::from("ab")).unwrap(), "8Qq");
    }

    #[test]
    fn test_encode_4() {
        let base = Base58;
        assert_eq!(base.encode(&String::from("abc")).unwrap(), "ZiCa");
    }

    #[test]
    fn test_decode_1() {
        let base = Base58;
        assert_eq!(
            base.decode(&String::from("2NEpo7TZRRrLZSi2U")).unwrap(),
            "Hello World!"
        );
    }

    #[test]
    fn test_decode_2() {
        let base = Base58;
        assert_eq!(base.decode(&String::from("2g")).unwrap(), "a");
    }

    #[test]
    fn test_decode_3() {
        let base = Base58;
        assert_eq!(base.decode(&String::from("8Qq")).unwrap(), "ab");
    }

    #[test]
    fn test_decode_4() {
        let base = Base58;
        assert_eq!(base.decode(&String::from("ZiCa")).unwrap(), "abc");
    }
}

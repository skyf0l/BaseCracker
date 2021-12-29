pub struct Base62;

use super::*;

impl Base for Base62 {
    fn get_name(&self) -> &'static str {
        "base62"
    }
    fn get_short_name(&self) -> &'static str {
        "b62"
    }
    fn get_base(&self) -> &'static str {
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
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
        let base = Base62;
        assert_eq!(
            base.encode(&String::from("Hello World!")).unwrap(),
            "T8dgcjRGkZ3aysdN"
        );
    }

    #[test]
    fn test_encode_2() {
        let base = Base62;
        assert_eq!(base.encode(&String::from("a")).unwrap(), "1Z");
    }

    #[test]
    fn test_encode_3() {
        let base = Base62;
        assert_eq!(base.encode(&String::from("ab")).unwrap(), "6U6");
    }

    #[test]
    fn test_encode_4() {
        let base = Base62;
        assert_eq!(base.encode(&String::from("abc")).unwrap(), "QmIN");
    }

    #[test]
    fn test_decode_1() {
        let base = Base62;
        assert_eq!(
            base.decode(&String::from("T8dgcjRGkZ3aysdN")).unwrap(),
            "Hello World!"
        );
    }

    #[test]
    fn test_decode_2() {
        let base = Base62;
        assert_eq!(base.decode(&String::from("1Z")).unwrap(), "a");
    }

    #[test]
    fn test_decode_3() {
        let base = Base62;
        assert_eq!(base.decode(&String::from("6U6")).unwrap(), "ab");
    }

    #[test]
    fn test_decode_4() {
        let base = Base62;
        assert_eq!(base.decode(&String::from("QmIN")).unwrap(), "abc");
    }
}

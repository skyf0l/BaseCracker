pub struct Base64;

impl super::Base for Base64 {
    fn encode(&self, decoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 encode: {}", decoded);
        Ok(decoded.to_string())
    }
    fn decode(&self, encoded: &String) -> Result<String, Box<dyn std::error::Error>> {
        println!("base64 decode: {}", encoded);
        Ok(encoded.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Base;
    use super::Base64;

    #[test]
    fn test_base64_encode() {
        let base64 = Base64;
        let result = base64.encode(&String::from("hello world"));
        assert_eq!(result.unwrap(), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode() {
        let base64 = Base64;
        let result = base64.decode(&String::from("aGVsbG8gd29ybGQ="));
        assert_eq!(result.unwrap(), "hello world");
    }
}

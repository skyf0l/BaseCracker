pub mod modules;
pub use modules::get_base_from_name;
pub use modules::get_bases_from_names;
pub use modules::get_bases_names;
use modules::Base;

pub mod utils;
pub use utils::get_printable_percentage;

/// Encode given plaintext using the specified bases and return the result
pub fn encode(plaintext: &str, bases: &Vec<Box<dyn Base>>) -> Result<String, String> {
    let mut result = plaintext.to_string();

    for base in bases {
        result = base.encode(&result)?;
    }
    Ok(result)
}

/// Encode given plaintext using the specified bases and return steps and result
/// It consumes more memory than encode()
///
/// Used for debugging
pub fn encode_steps(plaintext: &str, bases: &Vec<Box<dyn Base>>) -> Result<Vec<String>, String> {
    let mut result = vec![plaintext.to_string()];

    for base in bases {
        result.push(base.encode(&result[result.len() - 1])?);
    }

    // remove the first element (plaintext)
    result.remove(0);
    Ok(result)
}

/// Decode given cipher using the specified bases and return the result
pub fn decode(cipher: &str, bases: &Vec<Box<dyn Base>>) -> Result<String, String> {
    let mut result = cipher.to_string();

    for base in bases {
        result = base.decode(&result)?;
    }
    Ok(result)
}

/// Decode given cipher using the specified bases and return steps and result.
/// It consumes more memory than decode().
///
/// Used for debugging.
pub fn decode_steps(cipher: &str, bases: &Vec<Box<dyn Base>>) -> Result<Vec<String>, String> {
    let mut result = vec![cipher.to_string()];

    for base in bases {
        result.push(base.decode(&result[result.len() - 1])?);
    }

    // remove the first element (cipher)
    result.remove(0);
    Ok(result)
}

/// Iterate over all bases and try to decode the cipher.
/// Return a vector of (plaintext, bases) containing successful decodes.
pub fn crack_round(
    cipher: &str,
    bases: &Vec<Box<dyn Base>>,
    min_printable_percentage: f32,
) -> Vec<(String, String)> {
    let mut result = Vec::new();

    for base in bases {
        match base.decode(&cipher) {
            Ok(plain) => {
                if get_printable_percentage(&plain) >= min_printable_percentage {
                    result.push((plain, base.get_name().to_string()));
                }
            }
            Err(_) => {}
        }
    }
    result
}

/// Try to crack the cipher using the all bases.
pub fn basecracker(cipher: &str) -> Vec<(String, Vec<String>)> {
    basecracker_with_bases(cipher, &modules::get_bases())
}

/// Try to crack the cipher using the specified bases.
pub fn basecracker_with_bases(
    cipher: &str,
    bases: &Vec<Box<dyn Base>>,
) -> Vec<(String, Vec<String>)> {
    // exit if cipher is empty
    if cipher.len() == 0 {
        println!("Cipher is empty");
        return Vec::new();
    }

    let mut final_result = Vec::new();

    let min_printable_percentage = 0.9;

    // initialize the results vector
    let mut results = Vec::new();
    let round_result = crack_round(cipher, &bases, min_printable_percentage);
    for (plain, base) in round_result {
        results.push((plain, vec![base]));
    }

    // loop through the bases
    while results.len() > 0 {
        let mut next_results = Vec::new();
        for (plain, base_names) in results {
            let round_result = crack_round(&plain, &bases, min_printable_percentage);
            if round_result.len() == 0 {
                final_result.push((plain, base_names.clone()));
            } else {
                for (plain, base_name) in round_result {
                    let mut new_base_names = base_names.clone();
                    new_base_names.push(base_name);
                    next_results.push((plain, new_base_names));
                }
            }
        }
        results = next_results.clone();
    }

    final_result
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_crack_base64() {
        assert_eq!(
            basecracker("SGVsbG8gV29ybGQh"),
            vec![("Hello World!".to_string(), vec!["base64".to_string()])]
        );
    }

    #[test]
    fn test_encode_base64() {
        assert_eq!(
            encode(
                "Hello World!",
                &get_bases_from_names(&vec!["base64".to_string()]).unwrap()
            ),
            Ok("SGVsbG8gV29ybGQh".to_string())
        );
    }

    #[test]
    fn test_decode_base64() {
        assert_eq!(
            decode(
                "SGVsbG8gV29ybGQh",
                &get_bases_from_names(&vec!["base64".to_string()]).unwrap()
            ),
            Ok("Hello World!".to_string())
        );
    }

    #[test]
    fn test_crack_base64_x2() {
        assert_eq!(
            basecracker("U0dWc2JHOGdWMjl5YkdRaA=="),
            vec![(
                "Hello World!".to_string(),
                vec!["base64".to_string(), "base64".to_string()]
            )]
        );
    }

    #[test]
    fn test_encode_base64_x2() {
        assert_eq!(
            encode(
                "Hello World!",
                &get_bases_from_names(&vec!["base64".to_string(), "base64".to_string()]).unwrap()
            ),
            Ok("U0dWc2JHOGdWMjl5YkdRaA==".to_string())
        );
    }

    #[test]
    fn test_decode_base64_x2() {
        assert_eq!(
            decode(
                "U0dWc2JHOGdWMjl5YkdRaA==",
                &get_bases_from_names(&vec!["base64".to_string(), "base64".to_string()]).unwrap()
            ),
            Ok("Hello World!".to_string())
        );
    }

    #[test]
    fn test_crack_base64_x3() {
        assert_eq!(
            basecracker("VTBkV2MySkhPR2RXTWpsNVlrZFJhQT09"),
            vec![(
                "Hello World!".to_string(),
                vec![
                    "base64".to_string(),
                    "base64".to_string(),
                    "base64".to_string()
                ]
            )]
        );
    }

    #[test]
    fn test_encode_base64_x3() {
        assert_eq!(
            encode(
                "Hello World!",
                &get_bases_from_names(&vec![
                    "base64".to_string(),
                    "base64".to_string(),
                    "base64".to_string()
                ])
                .unwrap()
            ),
            Ok("VTBkV2MySkhPR2RXTWpsNVlrZFJhQT09".to_string())
        );
    }

    #[test]
    fn test_decode_base64_x3() {
        assert_eq!(
            decode(
                "VTBkV2MySkhPR2RXTWpsNVlrZFJhQT09",
                &get_bases_from_names(&vec![
                    "base64".to_string(),
                    "base64".to_string(),
                    "base64".to_string()
                ])
                .unwrap()
            ),
            Ok("Hello World!".to_string())
        );
    }

    #[test]
    fn test_crack_base64_hex_base58() {
        assert_eq!(
            basecracker("4afto9ow5ffzGyMCjboUeq5HbDkPTXpqPX4NMBwUCypB"),
            vec![(
                "Hello World!".to_string(),
                vec![
                    "base58".to_string(),
                    "hex".to_string(),
                    "base64".to_string()
                ]
            )]
        );
    }

    #[test]
    fn test_encode_base64_hex_base58() {
        assert_eq!(
            encode(
                "Hello World!",
                &get_bases_from_names(&vec![
                    "base64".to_string(),
                    "hex".to_string(),
                    "base58".to_string()
                ])
                .unwrap()
            ),
            Ok("4afto9ow5ffzGyMCjboUeq5HbDkPTXpqPX4NMBwUCypB".to_string())
        );
    }

    #[test]
    fn test_encode_steps_base64_hex_base58() {
        assert_eq!(
            encode_steps(
                "Hello World!",
                &get_bases_from_names(&vec![
                    "base64".to_string(),
                    "hex".to_string(),
                    "base58".to_string()
                ])
                .unwrap()
            ),
            Ok(vec![
                "SGVsbG8gV29ybGQh".to_string(),
                "53475673624738675632397962475168".to_string(),
                "4afto9ow5ffzGyMCjboUeq5HbDkPTXpqPX4NMBwUCypB".to_string()
            ])
        );
    }

    #[test]
    fn test_decode_base64_hex_base58() {
        assert_eq!(
            decode(
                "4afto9ow5ffzGyMCjboUeq5HbDkPTXpqPX4NMBwUCypB",
                &get_bases_from_names(&vec![
                    "base58".to_string(),
                    "hex".to_string(),
                    "base64".to_string()
                ])
                .unwrap()
            ),
            Ok("Hello World!".to_string())
        );
    }

    #[test]
    fn test_decode_steps_base64_hex_base58() {
        assert_eq!(
            decode_steps(
                "4afto9ow5ffzGyMCjboUeq5HbDkPTXpqPX4NMBwUCypB",
                &get_bases_from_names(&vec![
                    "base58".to_string(),
                    "hex".to_string(),
                    "base64".to_string()
                ])
                .unwrap()
            ),
            Ok(vec![
                "53475673624738675632397962475168".to_string(),
                "SGVsbG8gV29ybGQh".to_string(),
                "Hello World!".to_string()
            ])
        );
    }
}

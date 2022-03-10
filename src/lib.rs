pub mod modules;
pub use modules::get_base_from_name;
pub use modules::get_bases_from_names;
pub use modules::get_bases_names;
use modules::Base;

pub mod utils;
pub use utils::get_printable_percentage;

pub fn decode_round(
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

pub fn encode(cipher: &str, bases: &Vec<Box<dyn Base>>) -> Result<String, String> {
    let mut result = cipher.to_string();

    for base in bases {
        result = base.encode(&result)?;
    }
    Ok(result)
}

pub fn decode(cipher: &str, bases: &Vec<Box<dyn Base>>) -> Result<String, String> {
    let mut result = cipher.to_string();

    for base in bases {
        result = base.decode(&result)?;
    }
    Ok(result)
}

pub fn basecracker(cipher: &str) -> Vec<(String, Vec<String>)> {
    basecracker_with_bases(cipher, &modules::get_bases())
}

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
    let round_result = decode_round(cipher, &bases, min_printable_percentage);
    for (plain, base) in round_result {
        results.push((plain, vec![base]));
    }

    // loop through the bases
    while results.len() > 0 {
        let mut next_results = Vec::new();
        for (plain, base_names) in results {
            let round_result = decode_round(&plain, &bases, min_printable_percentage);
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
    fn test_crack_base64_hex_base58() {
        assert_eq!(
            basecracker("SeM5ddt2Lgxo2LJCZskRpa86BBPR7L13PffVDCP6FaCF73vnp5kCU4euqUfd18GP"),
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
}

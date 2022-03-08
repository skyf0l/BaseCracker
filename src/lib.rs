pub mod modules;

fn get_printable_percentage(s: &str) -> f32 {
    if s.len() == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for c in s.chars() {
        if c.is_alphabetic() || ((c as u32) >= 32 && (c as u32) < 127) {
            sum += 1.0;
        }
    }
    sum / s.len() as f32
}

pub fn decode_round(cipher: String, min_printable_percentage: f32) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let bases = modules::get_bases();

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

pub fn basecracker(cipher: &str) -> Vec<(String, Vec<String>)> {
    // exit if cipher is empty
    if cipher.len() == 0 {
        println!("Cipher is empty");
        return Vec::new();
    }

    let mut final_result = Vec::new();

    let min_printable_percentage = 0.9;

    // initialize the results vector
    let mut results = Vec::new();
    let round_result = decode_round(cipher.to_string(), min_printable_percentage);
    for (plain, base) in round_result {
        results.push((plain, vec![base]));
    }

    // loop through the bases
    while results.len() > 0 {
        let mut next_results = Vec::new();
        for (plain, bases) in results {
            let round_result = decode_round(plain.to_string(), min_printable_percentage);
            if round_result.len() == 0 {
                final_result.push((plain, bases.clone()));
            } else {
                for (plain, base) in round_result {
                    let mut new_bases = bases.clone();
                    new_bases.push(base);
                    next_results.push((plain, new_bases));
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

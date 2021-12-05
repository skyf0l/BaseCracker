pub mod modules;

fn get_printable_percentage(s: &str) -> f32 {
    let mut sum = 0.0;
    for c in s.chars() {
        if c.is_alphabetic() || ((c as u32) >= 32 && (c as u32) < 127) {
            sum += 1.0;
        }
    }
    sum / s.len() as f32
}

pub fn basecracker(cipher: &String) -> String {
    // exit if cipher is empty
    if cipher.len() == 0 {
        println!("Cipher is empty");
        return String::from("");
    }

    let mut result = String::new();
    let bases = modules::get_bases();

    for base in bases {
        match base.decode(&cipher) {
            Ok(plain) => {
                result = plain;
                println!(
                    "{}: {:.2}%: {}",
                    base.get_name(),
                    get_printable_percentage(result.as_str()) * 100.0,
                    result
                );
            }
            Err(e) => println!("{}: {}", base.get_name(), e),
        }
    }
    result
}

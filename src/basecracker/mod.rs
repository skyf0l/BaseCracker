mod modules;

pub fn basecracker(cipher: &String) -> String {
    let mut result = cipher.clone();
    let bases = modules::get_bases();

    for base in bases {
        match base.decode(&result) {
            Ok(plain) => result = plain,
            Err(e) => println!("{}", e),
        }
    }
    result
}

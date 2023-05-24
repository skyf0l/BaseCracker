pub fn printable_percentage(s: &str) -> f32 {
    if s.is_empty() {
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

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test {
    use super::*;

    #[test]
    fn test_printable_percentage() {
        const TESTLIST: [(&str, f32); 5] = [
            ("", 0.0),
            ("Hello World!", 1.0),
            ("He\0lo W\0rl\0!", 0.75),
            ("H\0l\0o\0\0\0r\0d!", 0.5),
            ("He\0\0\0\0\0o\0\0\0\0", 0.25),
        ];

        for test in TESTLIST.iter() {
            let printable_percentage = printable_percentage(test.0);
            assert_eq!(
                printable_percentage, test.1,
                "For string {}: Expected {} but got {}",
                test.0, test.1, printable_percentage
            );
        }
    }
}

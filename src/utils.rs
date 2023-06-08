pub fn printable_percentage(bytes: &[u8]) -> f32 {
    if bytes.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for c in bytes {
        if *c >= 32 && *c < 127 {
            sum += 1.0;
        }
    }
    sum / bytes.len() as f32
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test {
    use super::*;

    #[test]
    fn test_printable_percentage() {
        const TESTLIST: [(&[u8], f32); 5] = [
            (b"", 0.0),
            (b"Hello World!", 1.0),
            (b"He\0lo W\0rl\0!", 0.75),
            (b"H\0l\0o\0\0\0r\0d!", 0.5),
            (b"He\0\0\0\0\0o\0\0\0\0", 0.25),
        ];

        for (test, exp) in TESTLIST.iter() {
            let printable_percentage = printable_percentage(test);
            assert_eq!(
                printable_percentage,
                *exp,
                "For string {}: Expected {exp} but got {printable_percentage}",
                unsafe { std::str::from_utf8_unchecked(test) }
            );
        }
    }
}

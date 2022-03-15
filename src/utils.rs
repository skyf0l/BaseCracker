pub fn get_printable_percentage(s: &str) -> f32 {
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

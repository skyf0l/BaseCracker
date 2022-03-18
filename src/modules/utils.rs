pub use num_bigint::BigUint;
pub use num_traits::{One, ToPrimitive, Zero};
use std::cmp;
use std::ops::{Div, Rem};

pub trait PackedBy<T> {
    fn packed_by(&self, len: usize) -> Vec<T>;
}

impl PackedBy<String> for &str {
    fn packed_by(&self, len: usize) -> Vec<String> {
        let mut result = Vec::new();
        for i in (0..self.len()).step_by(len) {
            result.push(self[i..cmp::min(i + len, self.len())].to_string());
        }
        result
    }
}

pub fn str_to_int(s: &str, mult: u32) -> BigUint {
    let mut result = Zero::zero();
    for c in s.chars() {
        result = result * mult + BigUint::from(c as u32);
    }
    result
}

pub fn int_to_str(n: &BigUint, mult: u32) -> String {
    let mut result = String::new();
    let mut n = n.clone();
    while n > Zero::zero() {
        let q = (&n).div(BigUint::from(mult));
        let r = n.rem(BigUint::from(mult)).to_u32().unwrap();
        result.push(r as u8 as char);
        n = q;
    }
    result.chars().rev().collect()
}

pub fn to_base(n: &BigUint, base: &str, block_size: impl Into<Option<usize>>) -> String {
    let block_size = block_size.into().unwrap_or(1);

    let mut result = String::new();
    let mut n = n.clone();
    while n > Zero::zero() {
        let q = (&n).div(&BigUint::from(base.len()));
        let r = n.rem(&BigUint::from(base.len())).to_u32().unwrap();
        result.push(base.chars().nth(r as usize).unwrap());
        n = q;
    }
    if result.len() % block_size != 0 {
        result.push_str(&base[0..1].repeat(block_size - result.len() % block_size));
    }
    result.chars().rev().collect()
}

pub fn from_base(s: &str, base: &str) -> Result<BigUint, &'static str> {
    let mut result = Zero::zero();
    let mut power: BigUint = One::one();
    for c in s.chars().rev() {
        let index = base.find(c).ok_or("Invalid base")?;
        result += &power * &BigUint::from(index);
        power *= BigUint::from(base.len());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_packed_by_3_1() {
        let s = "abcdef";
        let result = s.packed_by(3);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "abc");
        assert_eq!(result[1], "def");
    }

    #[test]
    fn test_str_packed_by_3_2() {
        let s = "abcdefg";
        let result = s.packed_by(3);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "abc");
        assert_eq!(result[1], "def");
        assert_eq!(result[2], "g");
    }

    #[test]
    fn test_str_packed_by_3_3() {
        let s = "abcdefgh";
        let result = s.packed_by(3);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "abc");
        assert_eq!(result[1], "def");
        assert_eq!(result[2], "gh");
    }

    #[test]
    fn test_to_integer_1() {
        let result = str_to_int("Hello World!", 256);
        assert_eq!(result, BigUint::from(22405534230753928650781647905 as u128));
    }

    #[test]
    fn test_to_integer_2() {
        let result = str_to_int("BaseCracker", 256);
        assert_eq!(result, BigUint::from(80249302315773941590484338 as u128));
    }

    #[test]
    fn test_to_string_1() {
        let result = int_to_str(&BigUint::from(22405534230753928650781647905 as u128), 256);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_to_string_2() {
        let result = int_to_str(&BigUint::from(80249302315773941590484338 as u128), 256);
        assert_eq!(result, "BaseCracker");
    }

    #[test]
    fn test_to_base_2_1() {
        let n = BigUint::from(
            0b0110000101100010011000110110010001100101011001100110111001101110 as u64,
        );
        let base = "01".to_string();
        let result = to_base(&n, &base, None);
        assert_eq!(
            result,
            "110000101100010011000110110010001100101011001100110111001101110"
        );
    }
    #[test]
    fn test_to_base_2_2() {
        let n = BigUint::from(
            0b0110000101100010011000110110010001100101011001100110111001101110 as u64,
        );
        let base = "01".to_string();
        let result = to_base(&n, &base, 8);
        assert_eq!(
            result,
            "0110000101100010011000110110010001100101011001100110111001101110"
        );
    }

    #[test]
    fn test_to_base_16() {
        let n = BigUint::from(0x48656c6c6f20576f726c6421 as u128);
        let base = "0123456789abcdef".to_string();
        let result = to_base(&n, &base, 8);
        assert_eq!(result, "48656c6c6f20576f726c6421");
    }

    #[test]
    fn test_from_base_2_1() {
        let s = "110000101100010011000110110010001100101011001100110111001101110".to_string();
        let base = "01".to_string();
        let result = from_base(&s, &base);
        assert_eq!(
            result,
            Ok(BigUint::from(
                0b0110000101100010011000110110010001100101011001100110111001101110 as u64
            ))
        );
    }

    #[test]
    fn test_from_base_2_2() {
        let s = "0110000101100010011000110110010001100101011001100110111001101110".to_string();
        let base = "01".to_string();
        let result = from_base(&s, &base);
        assert_eq!(
            result,
            Ok(BigUint::from(
                0b0110000101100010011000110110010001100101011001100110111001101110 as u64
            ))
        );
    }

    #[test]
    fn test_from_base_16() {
        let s = "48656c6c6f20576f726c6421".to_string();
        let base = "0123456789abcdef".to_string();
        let result = from_base(&s, &base);
        assert_eq!(
            result,
            Ok(BigUint::from(0x48656c6c6f20576f726c6421 as u128))
        );
    }
}

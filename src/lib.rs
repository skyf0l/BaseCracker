#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

mod modules;
use iterator_ext::IteratorExt;
pub use modules::*;

/// Encodes the given plaintext using the specified bases and return the result as a vector of steps.
/// E.g. (plaintext, step1, step2, ..., ciphertext)
pub fn encode(plaintext: &str, bases: &[Box<dyn Base>]) -> Vec<String> {
    bases
        .iter()
        .scan(plaintext.to_string(), |text, base| {
            let encoded = base.encode(text);
            *text = encoded.clone();
            Some(encoded)
        })
        .collect()
}

/// Decodes the given ciphertext using the specified bases and return the result as a vector of steps.
/// E.g. (ciphertext, step1, step2, ..., plaintext)
pub fn decode(ciphertext: &str, bases: &[Box<dyn Base>]) -> Result<Vec<String>, DecodeError> {
    bases
        .iter()
        .map(Ok)
        .try_scan(ciphertext.to_string(), |acc, base| {
            let decoded = base.decode(acc)?;
            *acc = decoded.clone();
            Ok(Some(decoded))
        })
        .collect()
}

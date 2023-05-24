#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use iterator_ext::IteratorExt;

mod modules;
pub use modules::*;

mod tree;
mod utils;
use tree::{Node, Tree};

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

/// Crack data.
#[derive(Debug, Clone, PartialEq)]
pub struct CrackData {
    /// The base name used to decode the ciphertext.
    pub base_name: &'static str,
    /// The short name of the base used to decode the ciphertext.
    pub base_short_name: &'static str,
    /// The decoded text.
    pub decoded: String,
    /// The percentage of printable characters in the decoded text.
    pub printable_percentage: f32,
}

/// Crack tree.
pub type CrackTree = Tree<CrackData>;

/// Cracks the given ciphertext using the specified bases and return the result as a tree of steps.
pub fn crack(
    ciphertext: &str,
    bases: &[Box<dyn Base>],
    min_printable_percentage: f32,
) -> CrackTree {
    let mut tree = CrackTree::new(CrackData {
        base_name: "",
        base_short_name: "",
        decoded: ciphertext.to_string(),
        printable_percentage: utils::printable_percentage(&ciphertext),
    });

    crack_round(ciphertext, bases, min_printable_percentage, tree.root_mut());

    tree
}

/// Iterates over the given bases and generates a tree of all possible combinations of good bases.
pub fn crack_round(
    ciphertext: &str,
    bases: &[Box<dyn Base>],
    min_printable_percentage: f32,
    node: &mut Node<CrackData>,
) {
    for base in bases {
        let decoded = match base.decode(ciphertext) {
            Ok(decoded) => decoded,
            Err(_) => continue,
        };

        let printable_percentage = utils::printable_percentage(&decoded);

        if printable_percentage >= min_printable_percentage {
            let data = CrackData {
                base_name: base.get_metadata().name,
                base_short_name: base.get_metadata().short_name,
                decoded: decoded.clone(),
                printable_percentage,
            };

            let child = node.add_child(data);
            crack_round(&decoded, bases, min_printable_percentage, child);
        }
    }
}

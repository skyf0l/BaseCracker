#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use iterator_ext::IteratorExt;

mod modules;
pub use modules::*;

mod tree;
mod utils;
use tree::{RefNode, Tree};

/// Encodes the given plaintext using the specified bases and return the result as a vector of steps.
/// E.g. (plaintext, step1, step2, ..., ciphertext)
pub fn encode(plaintext: &str, bases: &[Box<dyn Base>]) -> Vec<String> {
    bases
        .iter()
        .scan(plaintext.to_string(), |text, base| {
            let encoded = base.encode(text.as_bytes());
            *text = encoded.clone();
            Some(encoded)
        })
        .collect()
}

/// Decodes the given ciphertext using the specified bases and return the result as a vector of steps.
/// E.g. (ciphertext, step1, step2, ..., plaintext)
pub fn decode(ciphertext: &str, bases: &[Box<dyn Base>]) -> Result<Vec<Vec<u8>>, DecodeError> {
    bases
        .iter()
        .map(Ok)
        .try_scan(ciphertext.as_bytes().to_vec(), |acc, base| {
            let decoded =
                base.decode(&String::from_utf8(acc.to_vec()).map_err(DecodeError::InvalidUtf8)?)?;
            *acc = decoded.clone();
            Ok(Some(decoded))
        })
        .collect()
}

/// Crack data.
#[derive(Debug, Clone, PartialEq)]
pub struct CrackData {
    /// The base used to decode the ciphertext.
    pub base: Option<&'static BaseMetadata>,
    /// The decoded data.
    pub decoded: Vec<u8>,
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
        base: None,
        decoded: ciphertext.as_bytes().to_vec(),
        printable_percentage: utils::printable_percentage(ciphertext.as_bytes()),
    });

    crack_round(ciphertext, bases, min_printable_percentage, tree.root());

    tree
}

/// Iterates over the given bases and generates a tree of all possible combinations of good bases.
pub fn crack_round(
    ciphertext: &str,
    bases: &[Box<dyn Base>],
    min_printable_percentage: f32,
    node: RefNode<CrackData>,
) {
    for base in bases {
        let decoded = match base.decode(ciphertext) {
            Ok(decoded) => decoded,
            Err(_) => continue,
        };

        let printable_percentage = utils::printable_percentage(&decoded);

        if printable_percentage >= min_printable_percentage {
            let data = CrackData {
                base: Some(base.get_metadata()),
                decoded: decoded.clone(),
                printable_percentage,
            };

            let child = tree::add_child(&node, data);
            crack_round(
                &String::from_utf8(decoded).unwrap(),
                bases,
                min_printable_percentage,
                child,
            );
        }
    }
}

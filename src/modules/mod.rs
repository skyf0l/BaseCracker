use std::string::FromUtf8Error;

use thiserror::Error;

mod module_base10;
mod module_base2;
mod module_base32;
mod module_base36;
mod module_base58;
mod module_base62;
mod module_base64;
mod module_base85;
mod module_hex;

/// Base Metadata.
/// It contains the name, short name, base, and padding of a base.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseMetadata {
    /// Name of the base.
    pub name: &'static str,
    /// Short name of the base.
    pub short_name: &'static str,
    /// Alphabet of the base.
    pub base: &'static str,
    /// Padding character of the base.
    pub padding: Option<&'static str>,
}

/// Errors that can occur while decoding.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    /// Padding characters (e.g. `=`) interspersed in the encoded form will be treated as invalid bytes.
    #[error("Invalid byte at offset {0}: {1:#04x}")]
    InvalidByte(usize, u8),
    /// The length of the input is invalid.
    #[error("Invalid length")]
    InvalidLength,
    /// The nature of the padding was not as configured: absent or incorrect when it must be
    /// canonical, or present when it must be absent, etc.
    #[error("Invalid padding")]
    InvalidPadding,
    /// The encoded string is not a valid UTF-8 string.
    #[error("Invalid UTF-8")]
    InvalidUtf8(#[from] FromUtf8Error),
    /// Generic error.
    #[error("Can not decode the provided data")]
    Error,
}

/// A base encoding/decoding module.
pub trait Base {
    /// Get the metadata of the base.
    fn get_metadata(&self) -> &'static BaseMetadata;

    /// Check if the encoded string looks like it is encoded with this base.
    /// This function does not check if the decoded string is actually valid.
    fn is_valid(&self, encoded: &str) -> bool {
        let metadata = self.get_metadata();
        let (base, padding) = (metadata.base, metadata.padding);

        let mut is_padding = false;

        for c in encoded.chars() {
            if is_padding {
                if padding.map_or(false, |p| !p.contains(c)) {
                    return false;
                }
            } else if !base.contains(c) {
                if padding.map_or(false, |p| p.contains(c)) {
                    is_padding = true;
                } else {
                    return false;
                }
            }
        }
        true
    }

    /// Encode a string.
    fn encode(&self, plain: &[u8]) -> String;

    /// Decode a string.
    fn decode(&self, enc: &str) -> Result<Vec<u8>, DecodeError>;
}

/// Get a list of all defined bases.
pub fn get_bases() -> Vec<Box<dyn Base>> {
    vec![
        Box::new(module_base2::Base2),
        Box::new(module_base10::Base10),
        Box::new(module_hex::Hex),
        Box::new(module_base32::Base32),
        Box::new(module_base36::Base36),
        Box::new(module_base58::Base58),
        Box::new(module_base62::Base62),
        Box::new(module_base64::Base64),
        Box::new(module_base85::Base85),
    ]
}

/// Get a list of all defined bases' names and short names.
pub fn get_bases_names() -> Vec<(String, String)> {
    get_bases()
        .into_iter()
        .map(|base| {
            (
                base.get_metadata().name.to_string(),
                base.get_metadata().short_name.to_string(),
            )
        })
        .collect()
}

/// Errors that can occur while getting a base.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BaseError {
    /// The base was not found.
    #[error("Base not found: {0}")]
    NotFound(String),
}

/// Get a base from its name or short name.
pub fn get_base_from_name(name: &str) -> Result<Box<dyn Base>, BaseError> {
    get_bases()
        .into_iter()
        .find(|base| base.get_metadata().name == name || base.get_metadata().short_name == name)
        .ok_or_else(|| BaseError::NotFound(name.to_string()))
}

/// Get a list of bases from a list of names or short names.
pub fn get_bases_from_names(names: &[String]) -> Result<Vec<Box<dyn Base>>, BaseError> {
    names.iter().map(|name| get_base_from_name(name)).collect()
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::modules::module_base64::Base64;

    #[test]
    fn test_base64_is_valid() {
        let base = Base64;
        assert!(base.is_valid(&String::from("YWJj")));
        assert!(base.is_valid(&String::from("aGVsbG8gd29ybGQ=")));
    }

    #[test]
    fn test_base64_is_not_valid() {
        let base = Base64;
        assert!(!base.is_valid(&String::from("YW%Jj")));
        assert!(!base.is_valid(&String::from("aGVsbG8gd29!ybGQ=")));
        assert!(!base.is_valid(&String::from("aGVsbG8gd29ybGQ=a")));
    }
}

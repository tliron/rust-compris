use super::super::*;

use {
    std::{io, num::*, string},
    thiserror::*,
};

//
// ParseError
//

/// Compris parse error.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// Hint.
    #[error("hint: {0}")]
    Hint(string::String),

    /// Reference not found.
    #[error("reference not found: {0}")]
    ReferenceNotFound(usize),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// UTF8.
    #[error("UTF8: {0}")]
    UTF8(#[from] string::FromUtf8Error),

    /// Parse integer.
    #[error("parse integer: {0}")]
    ParseInteger(#[from] ParseIntError),

    /// Parse float.
    #[error("parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),

    /// Saphyr (YAML).
    #[cfg(feature = "yaml")]
    #[error("Saphyr YAML: {0}")]
    Saphyr(#[from] saphyr_parser::ScanError),

    /// Struson (JSON).
    #[cfg(feature = "json")]
    #[error("Struson JSON: {0}")]
    Struson(#[from] struson::reader::ReaderError),

    /// Borc (CBOR).
    #[cfg(feature = "cbor")]
    #[error("Borc CBOR: {0}")]
    Borc(#[from] borc::errors::DecodeError),

    /// RMP (MessagePack).
    #[cfg(feature = "messagepack")]
    #[error("RMP MessagePack: {0}")]
    RMP(#[from] rmp::decode::ValueReadError),

    /// Base64.
    #[error("Base64: {0}")]
    Base64(#[from] base64::DecodeError),
}

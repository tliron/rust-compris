use super::super::*;

use {
    std::{io, num::*, string},
    thiserror::*,
};

//
// ReadError
//

/// Compris read error.
#[derive(Error, Debug)]
pub enum ReadError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// Hint.
    #[error("hint: {0}")]
    Hint(string::String),

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
    #[error("Saphyr YAML: {0}")]
    Saphyr(#[from] saphyr_parser::ScanError),

    /// Struson (JSON).
    #[error("Struson JSON: {0}")]
    Struson(#[from] struson::reader::ReaderError),

    /// Borc (CBOR).
    #[error("Borc CBOR: {0}")]
    Borc(#[from] borc::errors::DecodeError),

    /// RMP (MessagePack).
    #[error("RMP MessagePack: {0}")]
    RMP(#[from] rmp::decode::ValueReadError),

    /// Base64.
    #[error("Base64: {0}")]
    Base64(#[from] base64::DecodeError),
}

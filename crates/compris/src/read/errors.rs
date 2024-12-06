use super::super::*;

use {
    std::{io, num::*, string},
    thiserror::*,
};

//
// ReadError
//

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    #[error("hint: {0}")]
    Hint(string::String),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("UTF8: {0}")]
    UTF8(#[from] string::FromUtf8Error),

    #[error("parse integer: {0}")]
    ParseInteger(#[from] ParseIntError),

    #[error("parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Saphyr YAML: {0}")]
    Saphyr(#[from] saphyr_parser::ScanError),

    #[error("Struson JSON: {0}")]
    Struson(#[from] struson::reader::ReaderError),

    #[error("Borc CBOR: {0}")]
    Borc(#[from] borc::errors::DecodeError),

    #[error("RMP MessagePack: {0}")]
    RMP(#[from] rmp::decode::ValueReadError),

    #[error("Base64: {0}")]
    Base64(#[from] base64::DecodeError),
}

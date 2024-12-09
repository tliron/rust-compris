use super::super::*;

use {
    std::{fmt, io, string},
    thiserror::*,
};

//
// SerializationError
//

/// Compris serialization write error.
#[derive(Error, Debug)]
pub enum SerializationError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// UTF8.
    #[error("UTF8: {0}")]
    UTF8(#[from] string::FromUtf8Error),

    /// YAML.
    #[error("YAML: {0}")]
    YAML(#[from] serde_yml::Error),

    /// JSON.
    #[error("JSON: {0}")]
    JSON(#[from] struson::serde::SerializerError),

    /// XML.
    #[error("XML: {0}")]
    XML(#[from] serde_xml_rs::Error),

    /// CBOR.
    #[error("CBOR: {0}")]
    CBOR(#[from] CborWriteError),

    /// MessagePack.
    #[error("MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
}

//
// CborError
//

/// CBOR write error.
#[derive(Error, Debug)]
pub struct CborWriteError {
    borc: Option<borc::errors::EncodeError>,
    custom: string::String,
}

impl fmt::Display for CborWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.borc {
            Some(borc) => write!(formatter, "{:?}", borc),
            None => self.custom.fmt(formatter),
        }
    }
}

impl serde::ser::Error for CborWriteError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self { borc: None, custom: format!("{}", msg) }
    }
}

impl From<borc::errors::EncodeError> for CborWriteError {
    fn from(value: borc::errors::EncodeError) -> Self {
        Self { borc: Some(value), custom: string::String::new() }
    }
}

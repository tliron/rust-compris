use super::super::*;

use {
    std::{fmt, io, string},
    thiserror::*,
};

//
// WriteError
//

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("UTF8: {0}")]
    UTF8(#[from] string::FromUtf8Error),

    #[error("YAML: {0}")]
    YAML(#[from] serde_yml::Error),

    #[error("JSON: {0}")]
    JSON(#[from] struson::serde::SerializerError),

    #[error("XML: {0}")]
    XML(#[from] serde_xml_rs::Error),

    #[error("CBOR: {0}")]
    CBOR(#[from] CborError),

    #[error("MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
}

//
// CborError
//

#[derive(Error, Debug)]
pub struct CborError {
    borc: Option<borc::errors::EncodeError>,
    custom: string::String,
}

impl fmt::Display for CborError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.borc {
            Some(borc) => write!(formatter, "{:?}", borc),
            None => self.custom.fmt(formatter),
        }
    }
}

impl serde::ser::Error for CborError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self { borc: None, custom: format!("{}", msg) }
    }
}

impl From<borc::errors::EncodeError> for CborError {
    fn from(value: borc::errors::EncodeError) -> Self {
        Self { borc: Some(value), custom: string::String::new() }
    }
}

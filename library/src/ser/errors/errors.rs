use super::super::super::format::*;

#[cfg(feature = "cbor")]
use super::cbor::*;

use {
    std::{io, str},
    thiserror::*,
};

//
// SerializeError
//

/// Compris serialization error.
#[derive(Debug, Error)]
pub enum SerializeError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// UTF8.
    #[error("UTF8: {0}")]
    UTF8(#[from] str::Utf8Error),

    /// YAML.
    #[cfg(feature = "yaml")]
    #[error("YAML: {0}")]
    YAML(#[from] serde_yml::Error),

    /// JSON.
    #[cfg(feature = "json")]
    #[error("JSON: {0}")]
    JSON(#[from] struson::serde::SerializerError),

    /// XML.
    #[cfg(feature = "xml")]
    #[error("XML: {0}")]
    XML(#[from] serde_xml_rs::Error),

    /// CBOR.
    #[cfg(feature = "cbor")]
    #[error("CBOR: {0}")]
    CBOR(#[from] CborWriteError),

    /// MessagePack.
    #[cfg(feature = "messagepack")]
    #[error("MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
}

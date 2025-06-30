use super::super::{normal::*, *};

use {
    serde::de,
    std::{fmt, io},
    thiserror::*,
};

//
// DeserializeError
//

/// Compris deserialization error.
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// Incompatible type.
    #[error("incompatible type: {0}")]
    IncompatibleType(&'static str),

    /// Incompatible variant.
    #[error("incompatible variant: {0}")]
    IncompatibleVariant(String),

    /// No more items.
    #[error("no more items")]
    NoMoreItems,

    /// Not supported.
    #[error("not supported: {0}")]
    NotSupported(&'static str),

    /// Read.
    #[error("read: {0}")]
    Read(#[from] parse::ParseError),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Custom.
    #[error("custom: {0}")]
    Custom(String),
}

impl DeserializeError {
    /// Incompatible type.
    pub fn incompatible_type<AnnotatedT>(variant: &Variant<AnnotatedT>) -> DeserializeError {
        Self::IncompatibleType(variant.get_type_name())
    }

    /// Incompatible variant.
    pub fn incompatible_variant<AnnotatedT>(variant: &Variant<AnnotatedT>) -> DeserializeError {
        Self::IncompatibleVariant(format!("{}", variant))
    }
}

impl de::Error for DeserializeError {
    fn custom<DisplayableT>(message: DisplayableT) -> Self
    where
        DisplayableT: fmt::Display,
    {
        DeserializeError::Custom(message.to_string())
    }
}

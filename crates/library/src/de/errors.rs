use super::super::{normal::*, *};

use {
    std::{fmt, io},
    thiserror::*,
};

//
// DeserializationError
//

/// Deserialization error.
#[derive(Error, Debug)]
pub enum DeserializationError {
    /// Unsupported format.
    #[error("unsupported format: {0:?}")]
    UnsupportedFormat(Format),

    /// Incompatible type.
    #[error("incompatible type: {0}")]
    IncompatibleType(&'static str),

    /// Incompatible value.
    #[error("incompatible value: {0}")]
    IncompatibleValue(String),

    /// No more elements.
    #[error("no more elements")]
    NoMoreElements,

    /// Not supported.
    #[error("not supported: {0}")]
    NotSupported(&'static str),

    /// Read.
    #[error("read: {0}")]
    Read(#[from] read::ReadError),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Custom.
    #[error("custom: {0}")]
    Custom(String),
}

impl DeserializationError {
    /// Incompatible type.
    pub fn incompatible_type(value: &Value) -> DeserializationError {
        Self::IncompatibleType(value.get_type_name())
    }

    /// Incompatible value.
    pub fn incompatible_value(value: &Value) -> DeserializationError {
        Self::IncompatibleValue(format!("{}", value))
    }
}

impl serde::de::Error for DeserializationError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        DeserializationError::Custom(message.to_string())
    }
}

use super::super::*;

use {
    std::{fmt, io, string::String as StdString},
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
    IncompatibleValue(StdString),

    /// No more elements.
    #[error("no more elements")]
    NoMoreElements,

    /// Not supported.
    #[error("not supported: {0}")]
    NotSupported(&'static str),

    /// Custom.
    #[error("custom: {0}")]
    Custom(StdString),

    /// Read.
    #[error("read: {0}")]
    Read(#[from] read::ReadError),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),
}

impl DeserializationError {
    /// Incompatible type.
    pub fn incompatible_type(value: &Value) -> DeserializationError {
        Self::IncompatibleType(value.get_type_name())
    }

    /// Incompatible value.
    pub fn incompatible_value(value: &Value) -> Result<DeserializationError, io::Error> {
        Ok(Self::IncompatibleValue(value.to_debug_string()?))
    }
}

impl serde::de::Error for DeserializationError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        DeserializationError::Custom(message.to_string())
    }
}

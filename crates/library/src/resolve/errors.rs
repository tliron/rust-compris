use super::super::*;

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{error::*, fmt, io},
    thiserror::*,
};

//
// ResolveError
//

/// Resolve error.
///
/// The `OE` generic parameter is used to specify the type for [ResolveError::Other].
#[derive(Error, Debug)]
pub enum ResolveError<OE: Error = CustomError> {
    /// Wrong value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] LocatableError<IncompatibleValueTypeError>),

    /// Missing required key.
    #[error("missing required key: {0}")]
    MissingRequiredKey(#[from] LocatableError<MissingRequiredKeyError>),

    /// Unknown key.
    #[error("unknown key: {0}")]
    UnknownKey(#[from] LocatableError<UnknownKeyError>),

    /// Other errors.
    #[error("{0}")]
    Other(LocatableError<OE>),
}

impl<OE: Error + WriteDebug> WriteDebug for ResolveError<OE> {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> io::Result<()> {
        match self {
            ResolveError::IncompatibleValueType(wrong_value_type) => {
                wrong_value_type.write_debug_representation(writer, indentation, styles)
            }
            ResolveError::UnknownKey(unknown_key) => {
                unknown_key.write_debug_representation(writer, indentation, styles)
            }
            ResolveError::MissingRequiredKey(missing_required_key) => {
                missing_required_key.write_debug_representation(writer, indentation, styles)
            }
            ResolveError::Other(other) => other.write_debug_representation(writer, indentation, styles),
        }
    }
}

//
// MissingRequiredKeyError
//

/// Missing required key.
#[derive(Error, Debug)]
pub struct MissingRequiredKeyError {
    /// Key.
    pub key: Value,
}

impl MissingRequiredKeyError {
    /// Constructor.
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into() }
    }
}

impl WriteDebug for MissingRequiredKeyError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _indentation: usize,
        styles: &Styles,
    ) -> std::io::Result<()> {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "missing required key: {}", key.style(styles.error))
    }
}

impl fmt::Display for MissingRequiredKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key.fmt(formatter)
    }
}

//
// UnknownKeyError
//

/// Unknown key.
#[derive(Error, Debug)]
pub struct UnknownKeyError {
    /// Key.
    pub key: Value,
}

impl UnknownKeyError {
    /// Constructor.
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into() }
    }
}

impl WriteDebug for UnknownKeyError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _indentation: usize,
        styles: &Styles,
    ) -> std::io::Result<()> {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "unknown key: {}", key.style(styles.error))
    }
}

impl fmt::Display for UnknownKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key.fmt(formatter)
    }
}

//
// CustomError
//

/// Custom error.
#[derive(Error, Debug)]
pub struct CustomError {
    /// Message.
    pub message: String,
}

impl CustomError {
    /// Constructor.
    pub fn new(message: &str) -> Self {
        Self { message: message.into() }
    }
}

impl WriteDebug for CustomError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _indentation: usize,
        styles: &Styles,
    ) -> std::io::Result<()> {
        write!(writer, "{}", self.style(styles.error))
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, formatter)
    }
}

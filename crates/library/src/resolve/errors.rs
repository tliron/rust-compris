use super::super::{citation::*, normal::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MissingRequiredKeyError
//

/// Missing required key.
#[derive(Error, Debug)]
pub struct MissingRequiredKeyError {
    /// Key.
    pub key: Value,

    citation: Citation,
}

impl MissingRequiredKeyError {
    /// Constructor.
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into(), citation: Citation::default() }
    }
}

impl HasCitation for MissingRequiredKeyError {
    fn get_citation(&self) -> &Citation {
        &self.citation
    }

    fn with_citation(mut self, citation: Citation) -> Self {
        self.citation = citation;
        self
    }
}

impl Debuggable for MissingRequiredKeyError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _nested_prefix: &NestedPrefix,
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

    citation: Citation,
}

impl UnknownKeyError {
    /// Constructor.
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into(), citation: Citation::default() }
    }
}

impl HasCitation for UnknownKeyError {
    fn get_citation(&self) -> &Citation {
        &self.citation
    }

    fn with_citation(mut self, citation: Citation) -> Self {
        self.citation = citation;
        self
    }
}

impl Debuggable for UnknownKeyError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _nested_prefix: &NestedPrefix,
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

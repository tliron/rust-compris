use super::super::{cite::*, normal::*};

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
    pub fn new<KeyT>(key: KeyT) -> Self
    where
        KeyT: Into<Value>,
    {
        Self { key: key.into(), citation: Citation::default() }
    }
}

impl Citable for MissingRequiredKeyError {
    fn get_citation(&self) -> Option<&Citation> {
        Some(&self.citation)
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        Some(&mut self.citation)
    }
}

impl Debuggable for MissingRequiredKeyError {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        theme: &Theme,
    ) -> std::io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "missing required key: {}", key.style(theme.error))
    }
}

impl fmt::Display for MissingRequiredKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key.fmt(formatter)
    }
}

//
// InvalidKeyError
//

/// Invalid key.
#[derive(Error, Debug)]
pub struct InvalidKeyError {
    /// Key.
    pub key: Value,

    citation: Citation,
}

impl InvalidKeyError {
    /// Constructor.
    pub fn new<KeyT>(key: KeyT) -> Self
    where
        KeyT: Into<Value>,
    {
        Self { key: key.into(), citation: Citation::default() }
    }
}

impl Citable for InvalidKeyError {
    fn get_citation(&self) -> Option<&Citation> {
        Some(&self.citation)
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        Some(&mut self.citation)
    }
}

impl Debuggable for InvalidKeyError {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        theme: &Theme,
    ) -> std::io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "invalid key: {}", key.style(theme.error))
    }
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key.fmt(formatter)
    }
}

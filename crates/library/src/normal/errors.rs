use super::{super::citation::*, value::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
    thiserror::*,
};

//
// IncompatibleValueTypeError
//

/// Incompatible value type.
#[derive(Error, Debug)]
pub struct IncompatibleValueTypeError {
    /// Expected type name.
    pub expected_type_name: String,

    /// Type name.
    pub type_name: String,

    citation: Citation,
}

impl IncompatibleValueTypeError {
    /// Constructor.
    pub fn new(value: &Value, expected_type_name: &str) -> Self {
        Self {
            expected_type_name: expected_type_name.into(),
            type_name: value.get_type_name().into(),
            citation: Citation::default(),
        }
    }
}

impl HasCitation for IncompatibleValueTypeError {
    fn get_citation(&self) -> &Citation {
        &self.citation
    }

    fn with_citation(mut self, citation: Citation) -> Self {
        self.citation = citation;
        self
    }
}

impl Debuggable for IncompatibleValueTypeError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _nested_prefix: &NestedPrefix,
        styles: &Styles,
    ) -> std::io::Result<()> {
        write!(
            writer,
            "incompatible value type: is {}, expected {}",
            self.type_name.style(styles.error),
            self.expected_type_name
        )
    }
}

impl fmt::Display for IncompatibleValueTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_name)
    }
}

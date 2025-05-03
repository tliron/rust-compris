use super::{super::cite::*, value::*};

use {
    kutil_cli::debug::*,
    kutil_std::string::*,
    std::{fmt, io},
    thiserror::*,
};

//
// ConversionError
//

/// Conversion.
#[derive(Debug, Error)]
pub enum ConversionError {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] IncompatibleValueTypeError),

    /// Malformed.
    #[error("casting: {0}")]
    Casting(#[from] CastingError),
}

impl Citable for ConversionError {
    fn get_citation(&self) -> Option<&Citation> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_citation(),
            Self::Casting(casting) => casting.get_citation(),
        }
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_citation_mut(),
            Self::Casting(casting) => casting.get_citation_mut(),
        }
    }
}

impl Debuggable for ConversionError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => {
                incompatible_value_type.write_debug_for(writer, context)
            }
            Self::Casting(casting) => casting.write_debug_for(writer, context),
        }
    }
}

//
// IncompatibleValueTypeError
//

/// Incompatible value type.
#[derive(Debug, Error)]
pub struct IncompatibleValueTypeError {
    /// Expected type names.
    pub expected_type_names: Vec<String>,

    /// Type name.
    pub type_name: String,

    citation: Citation,
}

impl IncompatibleValueTypeError {
    /// Constructor.
    pub fn new(value: &Value, expected_type_names: &[&str]) -> Self {
        Self {
            expected_type_names: expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
            type_name: value.get_type_name().into(),
            citation: Citation::default(),
        }
    }
}

impl Citable for IncompatibleValueTypeError {
    fn get_citation(&self) -> Option<&Citation> {
        Some(&self.citation)
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        Some(&mut self.citation)
    }
}

impl Debuggable for IncompatibleValueTypeError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(
            writer,
            "incompatible value type: is {}, expected {}",
            context.theme.error(&self.type_name),
            self.expected_type_names.join_conjunction("or")
        )
    }
}

impl fmt::Display for IncompatibleValueTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}

//
// CastingError
//

/// Casting error.
#[derive(Debug, Error)]
pub struct CastingError {
    /// Value.
    pub value: String,

    /// Type name.
    pub type_name: String,

    citation: Citation,
}

impl CastingError {
    /// Constructor.
    pub fn new(value: &str, type_name: &str) -> Self {
        Self { value: value.into(), type_name: type_name.into(), citation: Citation::default() }
    }
}

impl Citable for CastingError {
    fn get_citation(&self) -> Option<&Citation> {
        Some(&self.citation)
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        Some(&mut self.citation)
    }
}

impl Debuggable for CastingError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} cannot be cast to a {}", self.value, context.theme.error(&self.type_name))
    }
}

impl fmt::Display for CastingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.value, self.type_name)
    }
}

//
// MalformedError
//

/// Malformed error.
#[derive(Debug, Error)]
pub struct MalformedError {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,

    citation: Citation,
}

impl MalformedError {
    /// Constructor.
    pub fn new(type_name: &str, reason: &str) -> Self {
        Self { type_name: type_name.into(), reason: reason.into(), citation: Citation::default() }
    }
}

impl Citable for MalformedError {
    fn get_citation(&self) -> Option<&Citation> {
        Some(&self.citation)
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        Some(&mut self.citation)
    }
}

impl Debuggable for MalformedError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "malformed: {}: {}", self.type_name, context.theme.error(&self.reason))
    }
}

impl fmt::Display for MalformedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}

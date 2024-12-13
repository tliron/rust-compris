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

    /// Requirement.
    pub requirement: Option<String>,

    citation: Citation,
}

impl IncompatibleValueTypeError {
    /// Constructor.
    pub fn new(value: &Value, expected_type_name: &str, requirement: Option<&str>) -> Self {
        Self {
            expected_type_name: expected_type_name.into(),
            type_name: value.get_type_name().into(),
            requirement: match requirement {
                Some(requirement) => Some(requirement.into()),
                None => None,
            },
            citation: Citation::default(),
        }
    }
}

impl Citable for IncompatibleValueTypeError {
    fn get_citation(&self) -> &Citation {
        &self.citation
    }

    fn with_citation(mut self, citation: Citation) -> Self {
        self.citation = citation;
        self
    }
}

impl Debuggable for IncompatibleValueTypeError {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        styles: &Styles,
    ) -> std::io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.requirement {
            Some(requirement) => write!(
                writer,
                "incompatible value type: is {}, expected {} {}",
                self.type_name.style(styles.error),
                self.expected_type_name,
                requirement,
            ),

            None => write!(
                writer,
                "incompatible value type: is {}, expected {}",
                self.type_name.style(styles.error),
                self.expected_type_name
            ),
        }
    }
}

impl fmt::Display for IncompatibleValueTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.requirement {
            Some(requirement) => {
                write!(formatter, "is {}, expected {} {}", self.type_name, self.expected_type_name, requirement)
            }

            None => write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_name),
        }
    }
}

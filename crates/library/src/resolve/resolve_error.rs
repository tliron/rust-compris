use super::{
    super::{citation::*, normal::*},
    errors::*,
};

use {kutil_cli::debug::*, std::io, thiserror::*};

//
// ResolveError
//

/// Resolve errors must be able to convert at least from the essential resolve errors.
pub trait ResolveError:
    HasCitation + From<IncompatibleValueTypeError> + From<MissingRequiredKeyError> + From<UnknownKeyError>
{
}

//
// CommonResolveError
//

/// Common resolve error.
///
/// An implementation of [ResolveError] that comprises the essential resolve errors.
#[derive(Error, Debug)]
pub enum CommonResolveError {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] IncompatibleValueTypeError),

    /// Missing required key.
    #[error("missing required key: {0}")]
    MissingRequiredKey(#[from] MissingRequiredKeyError),

    /// Unknown key.
    #[error("unknown key: {0}")]
    UnknownKey(#[from] UnknownKeyError),
}

impl ResolveError for CommonResolveError {}

// Delegated

impl HasCitation for CommonResolveError {
    fn get_citation(&self) -> &Citation {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_citation(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_citation(),
            Self::UnknownKey(unknown_key) => unknown_key.get_citation(),
        }
    }

    fn with_citation(self, citation: Citation) -> Self {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => {
                Self::IncompatibleValueType(incompatible_value_type.with_citation(citation))
            }
            Self::MissingRequiredKey(missing_required_key) => {
                Self::MissingRequiredKey(missing_required_key.with_citation(citation))
            }
            Self::UnknownKey(unknown_key) => Self::UnknownKey(unknown_key.with_citation(citation)),
        }
    }
}

impl Debuggable for CommonResolveError {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        nested_prefix: &NestedPrefix,
        styles: &Styles,
    ) -> io::Result<()> {
        match self {
            CommonResolveError::IncompatibleValueType(incompatible_value_type) => {
                incompatible_value_type.write_debug_representation(writer, nested_prefix, styles)
            }
            CommonResolveError::UnknownKey(unknown_key) => {
                unknown_key.write_debug_representation(writer, nested_prefix, styles)
            }
            CommonResolveError::MissingRequiredKey(missing_required_key) => {
                missing_required_key.write_debug_representation(writer, nested_prefix, styles)
            }
        }
    }
}

use super::super::{
    super::{cite::*, normal::*},
    error::*,
    errors::*,
};

use {kutil_cli::debug::*, std::io, thiserror::*};

//
// CommonResolveError
//

/// Common resolve error.
///
/// An implementation of [ResolveError] that comprises the essential resolve errors.
#[derive(Debug, Error)]
pub enum CommonResolveError {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] IncompatibleValueTypeError),

    /// Conversion.
    #[error("conversion: {0}")]
    Conversion(#[from] ConversionError),

    /// Malformed.
    #[error("malformed: {0}")]
    Malformed(#[from] MalformedError),

    /// Missing required key.
    #[error("missing required key: {0}")]
    MissingRequiredKey(#[from] MissingRequiredKeyError),

    /// Invalid key.
    #[error("invalid key: {0}")]
    InvalidKey(#[from] InvalidKeyError),
}

impl ResolveError for CommonResolveError {}

// Delegated

impl Citable for CommonResolveError {
    fn get_citation(&self) -> Option<&Citation> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_citation(),
            Self::Conversion(conversion) => conversion.get_citation(),
            Self::Malformed(malformed) => malformed.get_citation(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_citation(),
            Self::InvalidKey(invalid_key) => invalid_key.get_citation(),
        }
    }

    fn get_citation_mut(&mut self) -> Option<&mut Citation> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_citation_mut(),
            Self::Conversion(conversion) => conversion.get_citation_mut(),
            Self::Malformed(malformed) => malformed.get_citation_mut(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_citation_mut(),
            Self::InvalidKey(invalid_key) => invalid_key.get_citation_mut(),
        }
    }
}

impl Debuggable for CommonResolveError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => {
                incompatible_value_type.write_debug_for(writer, context)
            }
            Self::Conversion(conversion) => conversion.write_debug_for(writer, context),
            Self::Malformed(malformed) => malformed.write_debug_for(writer, context),
            Self::InvalidKey(invalid_key) => invalid_key.write_debug_for(writer, context),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.write_debug_for(writer, context),
        }
    }
}

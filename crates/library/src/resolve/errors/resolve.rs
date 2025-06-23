use super::{
    super::super::{annotation::*, normal::*},
    grammar::*,
    invalid_key::*,
    missing_required_key::*,
};

use {kutil_cli::debug::*, std::io, thiserror::*};

//
// ResolveError
//

/// [Resolve](super::super::resolve::Resolve) error.
#[derive(Debug, Error)]
pub enum ResolveError<AnnotationsT> {
    /// None.
    #[error("none")]
    None,

    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] IncompatibleValueTypeError<AnnotationsT>),

    /// Missing required key.
    #[error("missing required key: {0}")]
    MissingRequiredKey(#[from] MissingRequiredKeyError<AnnotationsT>),

    /// Invalid key.
    #[error("invalid key: {0}")]
    InvalidKey(#[from] InvalidKeyError<AnnotationsT>),

    /// Conversion.
    #[error("conversion: {0}")]
    Conversion(#[from] ConversionError<AnnotationsT>),

    /// Malformed.
    #[error("malformed: {0}")]
    Malformed(#[from] MalformedError<AnnotationsT>),

    /// Grammar.
    #[error("grammar: {0}")]
    Grammar(#[from] GrammarError<AnnotationsT>),
}

// Delegated

impl<AnnotationsT> Annotated for ResolveError<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::None => None,
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_annotations(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_annotations(),
            Self::InvalidKey(invalid_key) => invalid_key.get_annotations(),
            Self::Conversion(conversion) => conversion.get_annotations(),
            Self::Malformed(malformed) => malformed.get_annotations(),
            Self::Grammar(grammar) => grammar.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::None => None,
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_annotations_mut(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_annotations_mut(),
            Self::InvalidKey(invalid_key) => invalid_key.get_annotations_mut(),
            Self::Conversion(conversion) => conversion.get_annotations_mut(),
            Self::Malformed(malformed) => malformed.get_annotations_mut(),
            Self::Grammar(grammar) => grammar.get_annotations_mut(),
        }
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        match self {
            Self::None => {}
            Self::IncompatibleValueType(incompatible_value_type) => {
                incompatible_value_type.set_annotations(annotations)
            }
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.set_annotations(annotations),
            Self::InvalidKey(invalid_key) => invalid_key.set_annotations(annotations),
            Self::Conversion(conversion) => conversion.set_annotations(annotations),
            Self::Malformed(malformed) => malformed.set_annotations(annotations),
            Self::Grammar(grammar) => grammar.set_annotations(annotations),
        }
    }
}

impl<AnnotationsT> Debuggable for ResolveError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::None => write!(writer, "none"),
            Self::IncompatibleValueType(incompatible_value_type) => {
                incompatible_value_type.write_debug_for(writer, context)
            }
            Self::InvalidKey(invalid_key) => invalid_key.write_debug_for(writer, context),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.write_debug_for(writer, context),
            Self::Conversion(conversion) => conversion.write_debug_for(writer, context),
            Self::Malformed(malformed) => malformed.write_debug_for(writer, context),
            Self::Grammar(grammar) => grammar.write_debug_for(writer, context),
        }
    }
}

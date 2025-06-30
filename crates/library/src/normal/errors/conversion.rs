use super::{super::super::annotation::*, casting::*, incompatible_value_type::*};

use {kutil_cli::debug::*, thiserror::*};

//
// ConversionError
//

/// Conversion.
#[derive(Debug, Debuggable, Error)]
#[debuggable(variant = false)]
pub enum ConversionError<AnnotatedT> {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    #[debuggable(as(debuggable))]
    IncompatibleValueType(#[from] IncompatibleValueTypeError<AnnotatedT>),

    /// Malformed.
    #[error("casting: {0}")]
    #[debuggable(as(debuggable))]
    Casting(#[from] CastingError<AnnotatedT>),
}

impl<AnnotatedT> Annotated for ConversionError<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotatedT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_annotations(),
            Self::Casting(casting) => casting.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.get_annotations_mut(),
            Self::Casting(casting) => casting.get_annotations_mut(),
        }
    }

    fn set_annotations(&mut self, metadata: Annotations) {
        match self {
            Self::IncompatibleValueType(incompatible_value_type) => incompatible_value_type.set_annotations(metadata),
            Self::Casting(casting) => casting.set_annotations(metadata),
        }
    }
}

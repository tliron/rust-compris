use super::{super::super::annotation::*, casting::*, incompatible_value_type::*};

use {kutil_cli::debug::*, std::io, thiserror::*};

//
// ConversionError
//

/// Conversion.
#[derive(Debug, Error)]
pub enum ConversionError<AnnotationsT> {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    IncompatibleValueType(#[from] IncompatibleValueTypeError<AnnotationsT>),

    /// Malformed.
    #[error("casting: {0}")]
    Casting(#[from] CastingError<AnnotationsT>),
}

impl<AnnotationsT> Annotated for ConversionError<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
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

impl<AnnotationsT> Debuggable for ConversionError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
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

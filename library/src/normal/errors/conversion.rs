use super::{super::super::annotate::*, casting::*, incompatible_variant_type::*};

use {kutil::cli::depict::*, thiserror::*};

//
// ConversionError
//

/// Conversion.
#[derive(Debug, Depict, Error)]
#[depict(variant = false)]
pub enum ConversionError<AnnotatedT> {
    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    #[depict(as(depict))]
    IncompatibleVariantType(#[from] IncompatibleVariantTypeError<AnnotatedT>),

    /// Malformed.
    #[error("casting: {0}")]
    #[depict(as(depict))]
    Casting(#[from] CastingError<AnnotatedT>),
}

impl<AnnotatedT> ConversionError<AnnotatedT> {
    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> ConversionError<NewAnnotationsT>
    where
        AnnotatedT: Annotated + Default,
        NewAnnotationsT: Annotated + Default,
    {
        match self {
            Self::IncompatibleVariantType(incompatible_variant_type) => {
                incompatible_variant_type.into_annotated().into()
            }
            Self::Casting(casting) => casting.into_annotated().into(),
        }
    }
}

impl<AnnotatedT> Annotated for ConversionError<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        match self {
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.annotations(),
            Self::Casting(casting) => casting.annotations(),
        }
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.annotations_mut(),
            Self::Casting(casting) => casting.annotations_mut(),
        }
    }
}

use super::{
    super::super::{annotate::*, normal::*},
    invalid_key::*,
    missing_required_key::*,
};

use {kutil_cli::debug::*, std::fmt, thiserror::*};

//
// ResolveError
//

/// [Resolve](super::super::resolve::Resolve) error.
#[derive(Debug, Debuggable, Error)]
#[debuggable(variant = false)]
pub enum ResolveError<AnnotatedT> {
    /// Missing.
    #[error("missing")]
    #[debuggable(as(debuggable))]
    Missing,

    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    #[debuggable(as(debuggable))]
    IncompatibleVariantType(#[from] IncompatibleVariantTypeError<AnnotatedT>),

    /// Missing required key.
    #[error("missing required key: {0}")]
    #[debuggable(as(debuggable))]
    MissingRequiredKey(#[from] MissingRequiredKeyError<AnnotatedT>),

    /// Invalid key.
    #[error("invalid key: {0}")]
    #[debuggable(as(debuggable))]
    InvalidKey(#[from] InvalidKeyError<AnnotatedT>),

    /// Conversion.
    #[error("conversion: {0}")]
    #[debuggable(as(debuggable))]
    Conversion(#[from] ConversionError<AnnotatedT>),

    /// Malformed.
    #[error("malformed: {0}")]
    #[debuggable(as(debuggable))]
    Malformed(#[from] MalformedError<AnnotatedT>),

    /// Other.
    #[error("{0}")]
    #[debuggable(as(dyn_debuggable))]
    Other(CapturedAnnotatedError),
}

impl<AnnotatedT> ResolveError<AnnotatedT> {
    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> ResolveError<NewAnnotationsT>
    where
        AnnotatedT: Annotated + Default,
        NewAnnotationsT: Annotated + Default,
    {
        match self {
            Self::Missing => ResolveError::Missing.into(),
            Self::IncompatibleVariantType(incompatible_variant_type) => {
                incompatible_variant_type.into_annotated().into()
            }
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.into_annotated().into(),
            Self::InvalidKey(invalid_key) => invalid_key.into_annotated().into(),
            Self::Conversion(conversion) => conversion.into_annotated().into(),
            Self::Malformed(malformed) => malformed.into_annotated().into(),
            Self::Other(other) => ResolveError::Other(other),
        }
    }
}

// Delegated

impl<AnnotatedT> Annotated for ResolveError<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn has_annotations() -> bool {
        AnnotatedT::has_annotations()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Missing => None,
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.get_annotations(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_annotations(),
            Self::InvalidKey(invalid_key) => invalid_key.get_annotations(),
            Self::Conversion(conversion) => conversion.get_annotations(),
            Self::Malformed(malformed) => malformed.get_annotations(),
            Self::Other(other) => other.dyn_get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Missing => None,
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.get_annotations_mut(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.get_annotations_mut(),
            Self::InvalidKey(invalid_key) => invalid_key.get_annotations_mut(),
            Self::Conversion(conversion) => conversion.get_annotations_mut(),
            Self::Malformed(malformed) => malformed.get_annotations_mut(),
            Self::Other(other) => other.dyn_get_annotations_mut(),
        }
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        match self {
            Self::Missing => {}
            Self::IncompatibleVariantType(incompatible_value_type) => {
                incompatible_value_type.set_annotations(annotations)
            }
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.set_annotations(annotations),
            Self::InvalidKey(invalid_key) => invalid_key.set_annotations(annotations),
            Self::Conversion(conversion) => conversion.set_annotations(annotations),
            Self::Malformed(malformed) => malformed.set_annotations(annotations),
            Self::Other(other) => other.dyn_set_annotations(annotations),
        }
    }
}

impl<AnnotatedT> From<String> for ResolveError<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + fmt::Debug + Default + Send + Sync,
{
    fn from(message: String) -> Self {
        Self::Other(AnnotatedMessageError::<AnnotatedT>::new(message).into())
    }
}

impl<AnnotatedT> From<&str> for ResolveError<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + fmt::Debug + Default + Send + Sync,
{
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}

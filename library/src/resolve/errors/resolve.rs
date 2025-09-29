use super::{
    super::super::{annotate::*, normal::*},
    invalid_key::*,
    missing_required_key::*,
};

use {kutil::cli::depict::*, std::fmt, thiserror::*};

//
// ResolveError
//

/// [Resolve](super::super::resolve::Resolve) error.
#[derive(Debug, Depict, Error)]
#[depict(variant = false)]
pub enum ResolveError<AnnotatedT> {
    /// Missing.
    #[error("missing")]
    #[depict(as(depict))]
    Missing,

    /// Incompatible value type.
    #[error("incompatible value type: {0}")]
    #[depict(as(depict))]
    IncompatibleVariantType(#[from] IncompatibleVariantTypeError<AnnotatedT>),

    /// Missing required key.
    #[error("missing required key: {0}")]
    #[depict(as(depict))]
    MissingRequiredKey(#[from] MissingRequiredKeyError<AnnotatedT>),

    /// Invalid key.
    #[error("invalid key: {0}")]
    #[depict(as(depict))]
    InvalidKey(#[from] InvalidKeyError<AnnotatedT>),

    /// Conversion.
    #[error("conversion: {0}")]
    #[depict(as(depict))]
    Conversion(#[from] ConversionError<AnnotatedT>),

    /// Malformed.
    #[error("malformed: {0}")]
    #[depict(as(depict))]
    Malformed(#[from] MalformedError<AnnotatedT>),

    /// Other.
    #[error("{0}")]
    #[depict(as(dyn_depict))]
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
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Missing => None,
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.annotations(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.annotations(),
            Self::InvalidKey(invalid_key) => invalid_key.annotations(),
            Self::Conversion(conversion) => conversion.annotations(),
            Self::Malformed(malformed) => malformed.annotations(),
            Self::Other(other) => other.dyn_annotations(),
        }
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Missing => None,
            Self::IncompatibleVariantType(incompatible_value_type) => incompatible_value_type.annotations_mut(),
            Self::MissingRequiredKey(missing_required_key) => missing_required_key.annotations_mut(),
            Self::InvalidKey(invalid_key) => invalid_key.annotations_mut(),
            Self::Conversion(conversion) => conversion.annotations_mut(),
            Self::Malformed(malformed) => malformed.annotations_mut(),
            Self::Other(other) => other.dyn_annotations_mut(),
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

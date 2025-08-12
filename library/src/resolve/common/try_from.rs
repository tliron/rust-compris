use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    kutil::std::error::*,
    std::{fmt, marker::*},
};

/// Resolve a [Variant] into a [TryFrom] via an intermediate.
pub fn resolve_try_from<TryFromT, IntermediateT, AnnotatedT, ErrorRecipientT>(
    variant: &Variant<AnnotatedT>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<TryFromT, AnnotatedT>
where
    for<'value> &'value Variant<AnnotatedT>: TryInto<IntermediateT>,
    for<'value> <&'value Variant<AnnotatedT> as TryInto<IntermediateT>>::Error: fmt::Display,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
    ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
{
    let intermediate: IntermediateT = match variant.try_into() {
        Ok(intermediate) => intermediate,

        Err(error) => {
            errors.give(
                MalformedError::new(tynm::type_name::<IntermediateT>(), error.to_string())
                    .with_annotations_from(variant),
            )?;
            return Ok(None);
        }
    };

    Ok(match intermediate.try_into() {
        Ok(resolved) => Some(resolved),

        Err(error) => {
            errors.give(
                MalformedError::new(tynm::type_name::<TryFromT>(), error.to_string()).with_annotations_from(variant),
            )?;
            None
        }
    })
}

//
// ResolveTryFrom
//

/// A wrapper for a [TryFrom] that implements [Resolve].
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveTryFrom<InnerT, IntermediateT> {
    /// Inner.
    pub inner: InnerT,

    intermediate: PhantomData<IntermediateT>,
}

impl<InnerT, IntermediateT> ResolveTryFrom<InnerT, IntermediateT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner, intermediate: PhantomData }
    }
}

impl<InnerT, IntermediateT, AnnotatedT> Resolve<ResolveTryFrom<InnerT, IntermediateT>, AnnotatedT>
    for Variant<AnnotatedT>
where
    for<'variant> &'variant Variant<AnnotatedT>: TryInto<IntermediateT>,
    for<'variant> <&'variant Variant<AnnotatedT> as TryInto<IntermediateT>>::Error: fmt::Display,
    InnerT: TryFrom<IntermediateT>,
    InnerT::Error: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveTryFrom<InnerT, IntermediateT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        resolve_try_from(self, errors).map(|resolved| resolved.map(ResolveTryFrom::new))
    }
}

impl<InnerT, IntermediateT> AsRef<InnerT> for ResolveTryFrom<InnerT, IntermediateT> {
    fn as_ref(&self) -> &InnerT {
        &self.inner
    }
}

impl<InnerT, IntermediateT> From<InnerT> for ResolveTryFrom<InnerT, IntermediateT> {
    fn from(inner: InnerT) -> Self {
        Self::new(inner)
    }
}

impl<InnerT, IntermediateT> fmt::Display for ResolveTryFrom<InnerT, IntermediateT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

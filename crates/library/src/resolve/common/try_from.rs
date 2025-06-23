use super::super::{
    super::{annotation::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    kutil_std::error::*,
    std::{fmt, marker::*},
};

/// Resolve a [Value] into a [TryFrom] via an intermediate.
pub fn resolve_try_from<TryFromT, IntermediateT, AnnotationsT, ErrorRecipientT>(
    value: &Value<AnnotationsT>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<TryFromT, AnnotationsT>
where
    for<'value> &'value Value<AnnotationsT>: TryInto<IntermediateT>,
    for<'value> <&'value Value<AnnotationsT> as TryInto<IntermediateT>>::Error: fmt::Debug,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    AnnotationsT: Annotated + Clone + Default,
    ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
{
    let intermediate: IntermediateT = value.try_into().unwrap();

    Ok(match intermediate.try_into() {
        Ok(resolved) => Some(resolved),

        Err(error) => {
            errors.give(
                MalformedError::new(&tynm::type_name::<TryFromT>(), &error.to_string()).with_annotations_from(value),
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
pub struct ResolveTryFrom<TryFromT, IntermediateT> {
    /// Value.
    pub value: TryFromT,

    intermediate: PhantomData<IntermediateT>,
}

impl<TryFromT, IntermediateT> ResolveTryFrom<TryFromT, IntermediateT> {
    /// Constructor.
    pub fn new(value: TryFromT) -> Self {
        Self { value, intermediate: PhantomData }
    }
}

impl<TryFromT, IntermediateT> AsRef<TryFromT> for ResolveTryFrom<TryFromT, IntermediateT> {
    fn as_ref(&self) -> &TryFromT {
        &self.value
    }
}

impl<TryFromT, IntermediateT> From<TryFromT> for ResolveTryFrom<TryFromT, IntermediateT> {
    fn from(value: TryFromT) -> Self {
        Self::new(value)
    }
}

impl<TryFromT, IntermediateT> fmt::Display for ResolveTryFrom<TryFromT, IntermediateT>
where
    TryFromT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

impl<TryFromT, IntermediateT, AnnotationsT> Resolve<ResolveTryFrom<TryFromT, IntermediateT>, AnnotationsT>
    for Value<AnnotationsT>
where
    for<'value> &'value Value<AnnotationsT>: TryInto<IntermediateT>,
    for<'value> <&'value Value<AnnotationsT> as TryInto<IntermediateT>>::Error: fmt::Debug,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveTryFrom<TryFromT, IntermediateT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        resolve_try_from(self, errors).map(|resolved| resolved.map(ResolveTryFrom::new))
    }
}

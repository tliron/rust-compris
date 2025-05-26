use super::super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*};

use {
    kutil_std::error::*,
    std::{fmt, marker::*},
};

/// Resolve a [Value] into a [TryFrom] via an intermediate.
pub fn resolve_try_from<TryFromT, IntermediateT, ContextT, ErrorT, ErrorRecipientT>(
    value: &Value,
    context: Option<&ContextT>,
    ancestor: Option<&Value>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<TryFromT, ErrorT>
where
    for<'value> &'value Value: TryInto<IntermediateT>,
    for<'value> <&'value Value as TryInto<IntermediateT>>::Error: fmt::Debug,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    let intermediate: IntermediateT = value.try_into().unwrap();

    Ok(match intermediate.try_into() {
        Ok(resolved) => Some(resolved),

        Err(error) => {
            errors.give(
                MalformedError::new(&tynm::type_name::<TryFromT>(), &error.to_string())
                    .with_citation_for(value, context, ancestor),
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

impl<TryFromT, IntermediateT, ContextT, ErrorT> Resolve<ResolveTryFrom<TryFromT, IntermediateT>, ContextT, ErrorT>
    for Value
where
    for<'value> &'value Value: TryInto<IntermediateT>,
    for<'value> <&'value Value as TryInto<IntermediateT>>::Error: fmt::Debug,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveTryFrom<TryFromT, IntermediateT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        resolve_try_from(self, context, ancestor, errors).map(|resolved| resolved.map(ResolveTryFrom::new))
    }
}

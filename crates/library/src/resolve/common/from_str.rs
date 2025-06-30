use super::super::{
    super::{annotation::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    kutil_std::error::*,
    std::{fmt, str::*},
};

/// Resolve a [Value] into a [FromStr].
pub fn resolve_from_str<FromStrT, AnnotatedT, ErrorRecipientT>(
    value: &Value<AnnotatedT>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<FromStrT, AnnotatedT>
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
    ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
{
    Ok(match value {
        Value::Text(text) => match text.inner.parse() {
            Ok(parsed) => Some(parsed),

            Err(error) => {
                errors.give(
                    MalformedError::new(tynm::type_name::<FromStrT>(), error.to_string()).with_annotations_from(value),
                )?;
                None
            }
        },

        _ => {
            errors.give(IncompatibleValueTypeError::new(value, &["text"]).with_annotations_from(value))?;
            None
        }
    })
}

/// Implement [Resolve] for a [FromStr].
#[macro_export]
macro_rules! impl_resolve_from_str {
    ( $type:ident ) => {
        impl<AnnotatedT> $crate::resolve::Resolve<$type, AnnotatedT> for $crate::normal::Value<AnnotatedT>
        where
            AnnotatedT: $crate::annotation::Annotated + ::std::clone::Clone + ::std::default::Default,
        {
            fn resolve_with_errors<ErrorRecipientT>(
                &self,
                errors: &mut ErrorRecipientT,
            ) -> $crate::resolve::ResolveResult<$type, AnnotatedT>
            where
                ErrorRecipientT: ::kutil_std::error::ErrorRecipient<$crate::resolve::ResolveError<AnnotatedT>>,
            {
                $crate::resolve::resolve_from_str(self, errors)
            }
        }
    };
}

//
// ResolveFromStr
//

/// A wrapper for a [FromStr] that implements [Resolve].
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveFromStr<InnerT> {
    /// Inner.
    pub inner: InnerT,
}

impl<InnerT> ResolveFromStr<InnerT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner }
    }
}

impl<InnerT> AsRef<InnerT> for ResolveFromStr<InnerT> {
    fn as_ref(&self) -> &InnerT {
        &self.inner
    }
}

impl<InnerT> From<InnerT> for ResolveFromStr<InnerT> {
    fn from(value: InnerT) -> Self {
        Self::new(value)
    }
}

impl<InnerT> fmt::Display for ResolveFromStr<InnerT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

impl<InnerT, AnnotatedT> Resolve<ResolveFromStr<InnerT>, AnnotatedT> for Value<AnnotatedT>
where
    InnerT: FromStr,
    InnerT::Err: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveFromStr<InnerT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        resolve_from_str(self, errors).map(|resolved| resolved.map(ResolveFromStr::new))
    }
}

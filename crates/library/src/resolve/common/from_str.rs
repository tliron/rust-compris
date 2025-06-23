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
pub fn resolve_from_str<FromStrT, AnnotationsT, ErrorRecipientT>(
    value: &Value<AnnotationsT>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<FromStrT, AnnotationsT>
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    AnnotationsT: Annotated + Clone + Default,
    ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
{
    Ok(match value {
        Value::Text(text) => match text.value.parse() {
            Ok(parsed) => Some(parsed),

            Err(error) => {
                errors.give(
                    MalformedError::new(&tynm::type_name::<FromStrT>(), &error.to_string())
                        .with_annotations_from(value),
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
        impl<AnnotationsT> $crate::resolve::Resolve<$type, AnnotationsT> for $crate::normal::Value<AnnotationsT>
        where
            AnnotationsT: $crate::annotation::Annotated + ::std::clone::Clone + ::std::default::Default,
        {
            fn resolve_with_errors<ErrorRecipientT>(
                &self,
                errors: &mut ErrorRecipientT,
            ) -> $crate::resolve::ResolveResult<$type, AnnotationsT>
            where
                ErrorRecipientT: ::kutil_std::error::ErrorRecipient<$crate::resolve::ResolveError<AnnotationsT>>,
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
pub struct ResolveFromStr<FromStrT> {
    /// Value.
    pub value: FromStrT,
}

impl<FromStrT> ResolveFromStr<FromStrT> {
    /// Constructor.
    pub fn new(value: FromStrT) -> Self {
        Self { value }
    }
}

impl<FromStrT> AsRef<FromStrT> for ResolveFromStr<FromStrT> {
    fn as_ref(&self) -> &FromStrT {
        &self.value
    }
}

impl<FromStrT> From<FromStrT> for ResolveFromStr<FromStrT> {
    fn from(value: FromStrT) -> Self {
        Self::new(value)
    }
}

impl<FromStrT> fmt::Display for ResolveFromStr<FromStrT>
where
    FromStrT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

impl<FromStrT, AnnotationsT> Resolve<ResolveFromStr<FromStrT>, AnnotationsT> for Value<AnnotationsT>
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveFromStr<FromStrT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        resolve_from_str(self, errors).map(|resolved| resolved.map(ResolveFromStr::new))
    }
}

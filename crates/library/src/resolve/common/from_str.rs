use super::super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*};

use {
    kutil_std::error::*,
    std::{fmt, str::*},
};

/// Resolve a [Value] into a [FromStr].
pub fn resolve_from_str<FromStrT, ContextT, ErrorT, ErrorRecipientT>(
    value: &Value,
    context: Option<&ContextT>,
    ancestor: Option<&Value>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<FromStrT, ErrorT>
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    Ok(match value {
        Value::Text(text) => match text.value.parse() {
            Ok(parsed) => Some(parsed),

            Err(error) => {
                errors.give(
                    MalformedError::new(&tynm::type_name::<FromStrT>(), &error.to_string())
                        .with_citation_for(value, context, ancestor),
                )?;
                None
            }
        },

        _ => {
            errors
                .give(IncompatibleValueTypeError::new(value, &["text"]).with_citation_for(value, context, ancestor))?;
            None
        }
    })
}

/// Implement [Resolve] for a [FromStr].
#[macro_export]
macro_rules! impl_resolve_from_str {
    ( $type:ident ) => {
        impl<ContextT, ErrorT> $crate::resolve::Resolve<$type, ContextT, ErrorT> for $crate::normal::Value
        where
            ContextT: $crate::resolve::ResolveContext,
            ErrorT: $crate::resolve::ResolveError,
        {
            fn resolve_for<ErrorRecipientT>(
                &self,
                context: ::std::option::Option<&ContextT>,
                ancestor: ::std::option::Option<&$crate::normal::Value>,
                errors: &mut ErrorRecipientT,
            ) -> $crate::resolve::ResolveResult<$type, ErrorT>
            where
                ErrorRecipientT: ::kutil_std::error::ErrorRecipient<ErrorT>,
            {
                $crate::resolve::resolve_from_str(self, context, ancestor, errors)
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

impl<FromStrT, ContextT, ErrorT> Resolve<ResolveFromStr<FromStrT>, ContextT, ErrorT> for Value
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveFromStr<FromStrT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        resolve_from_str(self, context, ancestor, errors).map(|resolved| resolved.map(ResolveFromStr::new))
    }
}

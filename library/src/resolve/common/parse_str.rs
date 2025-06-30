use super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        resolve::*,
    },
    from_str::*,
};

use {
    kutil_std::{error::*, string::*},
    std::{fmt, marker::*, str::*},
};

//
// ResolveParseStr
//

/// A wrapper for a [ParseStr] that implements [Resolve].
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveParseStr<InnerT, ParseStrT> {
    /// Inner.
    pub inner: InnerT,

    resolve_parser: PhantomData<ParseStrT>,
}

impl<InnerT, ParseStrT> ResolveParseStr<InnerT, ParseStrT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner, resolve_parser: PhantomData }
    }
}

impl<InnerT, ParseStrT, AnnotatedT> Resolve<ResolveParseStr<InnerT, ParseStrT>, AnnotatedT> for Variant<AnnotatedT>
where
    ParseStrT: ParseStr<InnerT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveParseStr<InnerT, ParseStrT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        resolve_from_str(self, errors)
    }
}

impl<InnerT, ParseStrT> AsRef<InnerT> for ResolveParseStr<InnerT, ParseStrT> {
    fn as_ref(&self) -> &InnerT {
        &self.inner
    }
}

impl<InnerT, ParseStrT> From<InnerT> for ResolveParseStr<InnerT, ParseStrT> {
    fn from(inner: InnerT) -> Self {
        Self::new(inner)
    }
}

impl<InnerT, ParseStrT> FromStr for ResolveParseStr<InnerT, ParseStrT>
where
    ParseStrT: ParseStr<InnerT>,
{
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        ParseStrT::parse(representation).map(ResolveParseStr::new)
    }
}

impl<InnerT, ParseStrT> fmt::Display for ResolveParseStr<InnerT, ParseStrT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

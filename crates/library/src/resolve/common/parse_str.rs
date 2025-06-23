use super::{
    super::{
        super::{annotation::*, normal::*},
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
pub struct ResolveParseStr<ParsedT, ParseStrT> {
    /// Value.
    pub value: ParsedT,

    resolve_parser: PhantomData<ParseStrT>,
}

impl<ParsedT, ParseStrT> ResolveParseStr<ParsedT, ParseStrT> {
    /// Constructor.
    pub fn new(value: ParsedT) -> Self {
        Self { value, resolve_parser: PhantomData }
    }
}

impl<ParsedT, ParseStrT> AsRef<ParsedT> for ResolveParseStr<ParsedT, ParseStrT> {
    fn as_ref(&self) -> &ParsedT {
        &self.value
    }
}

impl<ParsedT, ParseStrT> From<ParsedT> for ResolveParseStr<ParsedT, ParseStrT> {
    fn from(value: ParsedT) -> Self {
        Self::new(value)
    }
}

impl<ParsedT, ParseStrT> FromStr for ResolveParseStr<ParsedT, ParseStrT>
where
    ParseStrT: ParseStr<ParsedT>,
{
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        ParseStrT::parse(representation).map(ResolveParseStr::new)
    }
}

impl<ParsedT, ParseStrT> fmt::Display for ResolveParseStr<ParsedT, ParseStrT>
where
    ParsedT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

impl<ParsedT, ParseStrT, AnnotationsT> Resolve<ResolveParseStr<ParsedT, ParseStrT>, AnnotationsT>
    for Value<AnnotationsT>
where
    ParseStrT: ParseStr<ParsedT>,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolveParseStr<ParsedT, ParseStrT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        resolve_from_str(self, errors)
    }
}

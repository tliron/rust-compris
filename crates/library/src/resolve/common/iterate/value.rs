use super::super::super::{super::normal::*, cite::*, context::*, error::*, iterator::*, resolve::*, result::*};

use {kutil_std::error::*, std::slice};

//
// ResolvingValueIterator
//

/// Resolves an [Iterator] of [Value], one item at a time.
///
/// Can be used directly on a [List].
///
/// Useful for implementing [Resolve] for list-like collections, such as [Vec].
pub struct ResolvingValueIterator<'own, IteratorT>
where
    IteratorT: Iterator<Item = &'own Value>,
{
    /// Iterator.
    pub iterator: IteratorT,
}

impl<'own, IteratorT> ResolvingValueIterator<'own, IteratorT>
where
    IteratorT: Iterator<Item = &'own Value>,
{
    /// Constructor.
    pub fn new(iterator: IteratorT) -> Self {
        Self { iterator }
    }

    /// Constructor.
    pub fn new_for<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<IntoIter = IteratorT>,
    {
        Self::new(iterable.into_iter())
    }
}

impl<'own> ResolvingValueIterator<'own, slice::Iter<'own, Value>> {
    /// Constructor.
    pub fn new_from<ContextT, ErrorT, ErrorRecipientT>(
        value: &'own Value,
        context: Option<&ContextT>,
        ancestor: Option<&'own Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, ErrorT>
    where
        ContextT: ResolveContext,
        ErrorT: ResolveError,
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        match value {
            Value::List(list) => return Ok(Some(Self::new_for(list))),

            _ => errors
                .give(IncompatibleValueTypeError::new(value, &["list"]).with_citation_for(value, context, ancestor))?,
        }

        Ok(None)
    }
}

impl<'own, ResolvedT, ContextT, ErrorT, IteratorT> ResolvingIterator<ResolvedT, ContextT, ErrorT>
    for ResolvingValueIterator<'own, IteratorT>
where
    Value: Resolve<ResolvedT, ContextT, ErrorT>,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    IteratorT: Iterator<Item = &'own Value>,
{
    fn resolve_next<ErrorRecipientT>(
        &mut self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        match self.iterator.next() {
            Some(value) => Ok(value.resolve_for(context, ancestor, errors)?),
            None => Ok(None),
        }
    }
}

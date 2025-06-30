use super::super::super::{
    super::{annotation::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use {kutil_std::error::*, std::slice};

//
// ResolvingValueIterator
//

/// Resolves an [Iterator] of [Value], one item at a time.
///
/// Can be used directly on a [List].
///
/// Useful for implementing [Resolve] for list-like collections, such as [Vec].
pub struct ResolvingValueIterator<'own, AnnotatedT, IteratorT>
where
    AnnotatedT: 'own,
    IteratorT: Iterator<Item = &'own Value<AnnotatedT>>,
{
    /// Iterator.
    pub iterator: IteratorT,
}

impl<'own, AnnotatedT, IteratorT> ResolvingValueIterator<'own, AnnotatedT, IteratorT>
where
    AnnotatedT: 'own,
    IteratorT: Iterator<Item = &'own Value<AnnotatedT>>,
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

impl<'own, AnnotatedT> ResolvingValueIterator<'own, AnnotatedT, slice::Iter<'own, Value<AnnotatedT>>> {
    /// Constructor.
    pub fn new_from<ErrorRecipientT>(
        value: &'own Value<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        match value {
            Value::List(list) => return Ok(Some(Self::new_for(list))),

            _ => errors.give(IncompatibleValueTypeError::new(value, &["list"]).with_annotations_from(value))?,
        }

        Ok(None)
    }
}

impl<'own, ResolvedT, AnnotatedT, IteratorT> ResolvingIterator<ResolvedT, AnnotatedT>
    for ResolvingValueIterator<'own, AnnotatedT, IteratorT>
where
    Value<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    IteratorT: Iterator<Item = &'own Value<AnnotatedT>>,
{
    fn resolve_next<ErrorRecipientT>(&mut self, errors: &mut ErrorRecipientT) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(match self.iterator.next() {
            Some(value) => value.resolve_with_errors(errors)?,
            None => None,
        })
    }
}

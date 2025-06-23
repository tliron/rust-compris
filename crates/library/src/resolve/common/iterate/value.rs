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
pub struct ResolvingValueIterator<'own, AnnotationsT, IteratorT>
where
    AnnotationsT: 'own,
    IteratorT: Iterator<Item = &'own Value<AnnotationsT>>,
{
    /// Iterator.
    pub iterator: IteratorT,
}

impl<'own, AnnotationsT, IteratorT> ResolvingValueIterator<'own, AnnotationsT, IteratorT>
where
    AnnotationsT: 'own,
    IteratorT: Iterator<Item = &'own Value<AnnotationsT>>,
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

impl<'own, AnnotationsT> ResolvingValueIterator<'own, AnnotationsT, slice::Iter<'own, Value<AnnotationsT>>> {
    /// Constructor.
    pub fn new_from<ErrorRecipientT>(
        value: &'own Value<AnnotationsT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotationsT>
    where
        AnnotationsT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        match value {
            Value::List(list) => return Ok(Some(Self::new_for(list))),

            _ => errors.give(IncompatibleValueTypeError::new(value, &["list"]).with_annotations_from(value))?,
        }

        Ok(None)
    }
}

impl<'own, ResolvedT, AnnotationsT, IteratorT> ResolvingIterator<ResolvedT, AnnotationsT>
    for ResolvingValueIterator<'own, AnnotationsT, IteratorT>
where
    Value<AnnotationsT>: Resolve<ResolvedT, AnnotationsT>,
    IteratorT: Iterator<Item = &'own Value<AnnotationsT>>,
{
    fn resolve_next<ErrorRecipientT>(&mut self, errors: &mut ErrorRecipientT) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        Ok(match self.iterator.next() {
            Some(value) => value.resolve_with_errors(errors)?,
            None => None,
        })
    }
}

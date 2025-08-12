use super::super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use {kutil::std::error::*, std::slice};

//
// ResolvingVariantIterator
//

/// Resolves an [Iterator] of [Variant], one item at a time.
///
/// Can be used directly on a [List].
///
/// Useful for implementing [Resolve] for list-like collections, such as [Vec].
pub struct ResolvingVariantIterator<'own, InnerT, AnnotatedT>
where
    AnnotatedT: 'own,
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
{
    /// Inner.
    pub inner: InnerT,
}

impl<'own, InnerT, AnnotatedT> ResolvingVariantIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
    AnnotatedT: 'own,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<IntoIter = InnerT>,
    {
        Self::new(iterable.into_iter())
    }
}

impl<'own, AnnotatedT> ResolvingVariantIterator<'own, slice::Iter<'own, Variant<AnnotatedT>>, AnnotatedT> {
    /// Constructor.
    pub fn new_from<ErrorRecipientT>(
        variant: &'own Variant<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        match variant {
            Variant::List(list) => return Ok(Some(Self::new_for(list))),

            _ => errors.give(IncompatibleVariantTypeError::new(variant, &["list"]))?,
        }

        Ok(None)
    }
}

impl<'own, ResolvedT, InnerT, AnnotatedT> ResolvingIterator<ResolvedT, AnnotatedT>
    for ResolvingVariantIterator<'own, InnerT, AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
{
    fn resolve_next<ErrorRecipientT>(&mut self, errors: &mut ErrorRecipientT) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(match self.inner.next() {
            Some(next) => next.resolve_with_errors(errors)?,
            None => None,
        })
    }
}

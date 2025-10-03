use super::super::super::{
    super::{annotate::*, kv::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use kutil::std::error::*;

//
// ResolvingKeyValuePairIterator
//

/// Resolves a [KeyValuePairIterator], one key-value pair at a time.
///
/// Both keys and values are resolved.
///
/// Note that the implementation relies on `dyn` to support different [KeyValuePairIterator]
/// implementations.
///
/// Useful for implementing [Resolve] for map-like collections, such as
/// [HashMap](std::collections::HashMap).
pub struct ResolvingKeyValuePairIterator<'own, AnnotatedT> {
    /// Inner key-value pair iterator.
    pub inner: Box<dyn IntoKeyValuePairIterator<AnnotatedT> + 'own>,
}

impl<'own, AnnotatedT> ResolvingKeyValuePairIterator<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: Box<dyn IntoKeyValuePairIterator<AnnotatedT> + 'own>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_from<ErrorRecipientT>(
        variant: Variant<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: 'own + Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        if variant.is_collection() {
            let iterator = variant.into_key_value_iterator().expect("map or list");
            Ok(Some(Self::new(iterator)))
        } else {
            errors.give(IncompatibleVariantTypeError::new_from(&variant, &["map", "list"]))?;
            Ok(None)
        }
    }
}

impl<'own, KeyT, ValueT, AnnotatedT> ResolvingIterator<(KeyT, ValueT), AnnotatedT>
    for ResolvingKeyValuePairIterator<'own, AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<KeyT, AnnotatedT>,
    Variant<AnnotatedT>: Resolve<ValueT, AnnotatedT>,
    AnnotatedT: Annotated + Default,
{
    fn resolve_next<ErrorRecipientT>(
        &mut self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<(KeyT, ValueT), AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        // Repeat until we get a non-error
        loop {
            match self.inner.next() {
                Ok(next) => {
                    return Ok(match next {
                        Some(pair) => pair.resolve_with_errors(errors)?,
                        None => None,
                    });
                }

                Err((error, cause)) => errors.give(error.with_annotations_from(&cause))?,
            }
        }
    }
}

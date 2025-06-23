use super::super::super::{
    super::{annotation::*, kv::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use kutil_std::error::*;

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
pub struct ResolvingKeyValuePairIterator<'own, AnnotationsT> {
    /// Key-value pair iterator.
    pub iterator: Box<dyn KeyValuePairIterator<AnnotationsT> + 'own>,
}

impl<'own, AnnotationsT> ResolvingKeyValuePairIterator<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(iterator: Box<dyn KeyValuePairIterator<AnnotationsT> + 'own>) -> Self {
        Self { iterator }
    }

    /// Constructor.
    pub fn new_from<ErrorRecipientT>(
        value: &'own Value<AnnotationsT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotationsT>
    where
        AnnotationsT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        match value.key_value_iterator() {
            Some(iterator) => Ok(Some(Self::new(iterator))),

            None => {
                errors.give(IncompatibleValueTypeError::new(value, &["map", "list"]).with_annotations_from(value))?;
                Ok(None)
            }
        }
    }
}

impl<'own, KeyT, ValueT, AnnotationsT> ResolvingIterator<(KeyT, ValueT), AnnotationsT>
    for ResolvingKeyValuePairIterator<'own, AnnotationsT>
where
    Value<AnnotationsT>: Resolve<KeyT, AnnotationsT>,
    Value<AnnotationsT>: Resolve<ValueT, AnnotationsT>,
    AnnotationsT: Annotated + Default,
{
    fn resolve_next<ErrorRecipientT>(
        &mut self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<(KeyT, ValueT), AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        // Repeat until we get a non-error
        loop {
            match self.iterator.next() {
                Ok(next) => {
                    return Ok(match next {
                        Some(pair) => pair.resolve_with_errors(errors)?,
                        None => None,
                    });
                }

                Err((error, cause)) => errors.give(error.with_annotations_from(cause))?,
            }
        }
    }
}

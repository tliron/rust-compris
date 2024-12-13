use super::super::super::{
    super::{kv::*, normal::*},
    cite::*,
    context::*,
    error::*,
    iterator::*,
    resolve::*,
    result::*,
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
pub struct ResolvingKeyValuePairIterator<'own> {
    /// Key-value pair iterator.
    pub iterator: Box<dyn KeyValuePairIterator + 'own>,
}

impl<'own> ResolvingKeyValuePairIterator<'own> {
    /// Constructor.
    pub fn new(iterator: Box<dyn KeyValuePairIterator + 'own>) -> Self {
        Self { iterator }
    }

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
        match value.key_value_iterator() {
            Some(iterator) => Ok(Some(Self::new(iterator))),

            None => {
                errors.give(
                    IncompatibleValueTypeError::new(value, &["map", "list"])
                        .with_citation_for(value, context, ancestor),
                )?;
                Ok(None)
            }
        }
    }
}

impl<'own, KeyT, ValueT, ContextT, ErrorT> ResolvingIterator<(KeyT, ValueT), ContextT, ErrorT>
    for ResolvingKeyValuePairIterator<'own>
where
    Value: Resolve<KeyT, ContextT, ErrorT>,
    Value: Resolve<ValueT, ContextT, ErrorT>,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_next<ErrorRecipientT>(
        &mut self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<(KeyT, ValueT), ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        // Repeat until we get a non-error
        loop {
            match self.iterator.next() {
                Ok(next) => {
                    return Ok(match next {
                        Some(pair) => pair.resolve_for(context, ancestor, errors)?,
                        None => None,
                    });
                }

                Err((error, cause)) => errors.give(error.with_citation_for(cause, context, ancestor))?,
            }
        }
    }
}

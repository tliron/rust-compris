use super::{super::normal::*, iterator::*};

use kutil_std::collections::*;

//
// KeyValuePairIteratorForValueIterator
//

/// A [KeyValuePairIterator] for an [Iterator] of [Value].
///
/// The iterated values are expected to be [List](super::super::normal::List)
/// of length 2 (key-value pairs).
///
/// Keeps track of keys and will report errors if it encounters duplicates.
pub struct KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Value<AnnotatedT>>,
{
    /// Inner iterator.
    pub inner: InnerT,

    /// Accumulated keys.
    pub keys: FastHashSet<&'own Value<AnnotatedT>>,
}

impl<'own, InnerT, AnnotatedT> KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Value<AnnotatedT>>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner, keys: FastHashSet::new() }
    }

    /// Constructor.
    pub fn new_for<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<IntoIter = InnerT>,
    {
        Self::new(iterable.into_iter())
    }
}

impl<'own, InnerT, AnnotatedT> KeyValuePairIterator<AnnotatedT>
    for KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Value<AnnotatedT>>,
    AnnotatedT: Default,
{
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Value<AnnotatedT>, &'own Value<AnnotatedT>)>,
        (MalformedError<AnnotatedT>, &Value<AnnotatedT>),
    > {
        if let Some(item) = self.inner.next() {
            if let Some((key, value)) = item.to_pair() {
                if self.keys.contains(key) {
                    return Err((MalformedError::new("key-value pair".into(), "key is not unique".into()), key));
                } else {
                    self.keys.insert(key);
                    return Ok(Some((key, value)));
                }
            }

            return Err((MalformedError::new("key-value pair".into(), "is not list of length 2".into()), item));
        }

        Ok(None)
    }
}

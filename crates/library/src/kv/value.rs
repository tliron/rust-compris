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
pub struct KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotationsT>
where
    InnerT: Iterator<Item = &'own Value<AnnotationsT>>,
{
    /// Inner iterator.
    pub inner: InnerT,

    /// Accumulated keys.
    pub keys: FastHashSet<&'own Value<AnnotationsT>>,
}

impl<'own, InnerT, AnnotationsT> KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotationsT>
where
    InnerT: Iterator<Item = &'own Value<AnnotationsT>>,
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

impl<'own, InnerT, AnnotationsT> KeyValuePairIterator<AnnotationsT>
    for KeyValuePairIteratorForValueIterator<'own, InnerT, AnnotationsT>
where
    InnerT: Iterator<Item = &'own Value<AnnotationsT>>,
    AnnotationsT: Default,
{
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Value<AnnotationsT>, &'own Value<AnnotationsT>)>,
        (MalformedError<AnnotationsT>, &Value<AnnotationsT>),
    > {
        if let Some(item) = self.inner.next() {
            if let Some((key, value)) = item.to_pair() {
                if self.keys.contains(key) {
                    return Err((MalformedError::new("key-value pair", "key is not unique"), key));
                } else {
                    self.keys.insert(key);
                    return Ok(Some((key, value)));
                }
            }

            return Err((MalformedError::new("key-value pair", "is not list of length 2"), item));
        }

        Ok(None)
    }
}

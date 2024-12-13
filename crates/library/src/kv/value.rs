use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForValueIterator
//

/// A [KeyValuePairIterator] for an [Iterator] of [Value].
///
/// The iterated values are expected to be [List](super::super::normal::List)
/// of length 2 (key-value pairs).
///
/// Keeps track of keys and will report errors if it encounters duplicates.
pub struct KeyValuePairIteratorForValueIterator<'own, IteratorT>
where
    IteratorT: Iterator<Item = &'own Value>,
{
    /// Value iterator.
    pub iterator: IteratorT,

    /// Accumulated keys.
    pub keys: HashSet<&'own Value>,
}

impl<'own, IteratorT> KeyValuePairIteratorForValueIterator<'own, IteratorT>
where
    IteratorT: Iterator<Item = &'own Value>,
{
    /// Constructor.
    pub fn new(iterator: IteratorT) -> Self {
        Self { iterator, keys: HashSet::new() }
    }

    /// Constructor.
    pub fn new_for<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<IntoIter = IteratorT>,
    {
        Self::new(iterable.into_iter())
    }
}

impl<'own, IteratorT> KeyValuePairIterator for KeyValuePairIteratorForValueIterator<'own, IteratorT>
where
    IteratorT: Iterator<Item = &'own Value>,
{
    fn next(&mut self) -> Result<Option<(&'own Value, &'own Value)>, (MalformedError, &Value)> {
        if let Some(item) = self.iterator.next() {
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

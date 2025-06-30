use super::{super::normal::*, iterator::*};

use kutil_std::collections::*;

//
// KeyValuePairIteratorForVariantIterator
//

/// A [KeyValuePairIterator] for an [Iterator] of [Variant].
///
/// The items are expected to be [List](super::super::normal::List) of length 2 (key-value pairs).
///
/// Keeps track of keys and will report errors if it encounters duplicates.
pub struct KeyValuePairIteratorForVariantIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
{
    /// Inner.
    pub inner: InnerT,

    /// Accumulated keys.
    pub keys: FastHashSet<&'own Variant<AnnotatedT>>,
}

impl<'own, InnerT, AnnotatedT> KeyValuePairIteratorForVariantIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner, keys: FastHashSet::default() }
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
    for KeyValuePairIteratorForVariantIterator<'own, InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = &'own Variant<AnnotatedT>>,
    AnnotatedT: Default,
{
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Variant<AnnotatedT>, &'own Variant<AnnotatedT>)>,
        (MalformedError<AnnotatedT>, &Variant<AnnotatedT>),
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

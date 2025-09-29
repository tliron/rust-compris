use super::{super::normal::*, iterator::*};

use kutil::std::collections::*;

//
// KeyValuePairIteratorForVariantIterator
//

/// A [KeyValuePairIterator] for an [Iterator] of [Variant].
///
/// The items are expected to be [List](super::super::normal::List) of length 2 (key-value pairs).
///
/// Keeps track of keys and will report errors if it encounters duplicates.
pub struct KeyValuePairIteratorForVariantIterator<'own, InnerT, AnnotatedT> {
    /// Inner.
    pub inner: InnerT,

    /// Accumulated keys.
    pub keys: FastHashSet<&'own Variant<AnnotatedT>>,
}

impl<'own, InnerT, AnnotatedT> KeyValuePairIteratorForVariantIterator<'own, InnerT, AnnotatedT> {
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

//
// IntoKeyValuePairIteratorForVariantIterator
//

/// An [IntoKeyValuePairIterator] for an [Iterator] of [Variant].
///
/// The items are expected to be [List](super::super::normal::List) of length 2 (key-value pairs).
///
/// Keeps track of keys and will report errors if it encounters duplicates.
pub struct IntoKeyValuePairIteratorForVariantIterator<InnerT, AnnotatedT> {
    /// Inner.
    pub inner: InnerT,

    /// Accumulated keys.
    pub keys: FastHashSet<Variant<AnnotatedT>>,
}

impl<InnerT, AnnotatedT> IntoKeyValuePairIteratorForVariantIterator<InnerT, AnnotatedT> {
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

impl<InnerT, AnnotatedT> IntoKeyValuePairIterator<AnnotatedT>
    for IntoKeyValuePairIteratorForVariantIterator<InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = Variant<AnnotatedT>>,
    AnnotatedT: Clone + Default,
{
    fn next(
        &mut self,
    ) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (MalformedError<AnnotatedT>, Variant<AnnotatedT>)>
    {
        if let Some(item) = self.inner.next() {
            if let Variant::List(list) = &item
                && list.inner.len() == 2
            {
                let (key, value) = item.into_pair().expect("list of length 2");
                if self.keys.contains(&key) {
                    return Err((MalformedError::new("key-value pair".into(), "key is not unique".into()), key));
                } else {
                    // TODO: any way we can test this without cloning?
                    self.keys.insert(key.clone());
                    return Ok(Some((key, value)));
                }
            }

            return Err((MalformedError::new("key-value pair".into(), "is not list of length 2".into()), item));
        }

        Ok(None)
    }
}

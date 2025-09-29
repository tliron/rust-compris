use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForHashMap
//

/// A [KeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForHashMap<'own, AnnotatedT> {
    /// Inner iterator.
    pub inner: hash_map::Iter<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<'own, AnnotatedT> KeyValuePairIteratorForHashMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: hash_map::Iter<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'own HashMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForHashMap<'own, AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Variant<AnnotatedT>, &'own Variant<AnnotatedT>)>,
        (MalformedError<AnnotatedT>, &Variant<AnnotatedT>),
    > {
        Ok(self.inner.next())
    }
}

//
// IntoKeyValuePairIteratorForHashMap
//

/// An [IntoKeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    /// Inner iterator.
    pub inner: hash_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<AnnotatedT> IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    /// Constructor.
    pub fn new(inner: hash_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: HashMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<AnnotatedT> IntoKeyValuePairIterator<AnnotatedT> for IntoKeyValuePairIteratorForHashMap<AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (MalformedError<AnnotatedT>, Variant<AnnotatedT>)>
    {
        Ok(self.inner.next())
    }
}

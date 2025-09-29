use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForBTreeMap
//

/// A [KeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForBTreeMap<'own, AnnotatedT> {
    /// Inner.
    pub inner: btree_map::Iter<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<'own, AnnotatedT> KeyValuePairIteratorForBTreeMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: btree_map::Iter<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'own BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForBTreeMap<'own, AnnotatedT> {
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
// KeyValuePairIteratorForBTreeMap
//

/// An [IntoKeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    /// Inner.
    pub inner: btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>,
}

impl<AnnotatedT> IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    /// Constructor.
    pub fn new(inner: btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<AnnotatedT> IntoKeyValuePairIterator<AnnotatedT> for IntoKeyValuePairIteratorForBTreeMap<AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)>, (MalformedError<AnnotatedT>, Variant<AnnotatedT>)>
    {
        Ok(self.inner.next())
    }
}

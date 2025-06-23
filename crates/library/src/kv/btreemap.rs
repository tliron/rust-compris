use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForBTreeMap
//

/// A [KeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForBTreeMap<'own, AnnotationsT> {
    /// Inner iterator.
    pub inner: btree_map::Iter<'own, Value<AnnotationsT>, Value<AnnotationsT>>,
}

impl<'own, AnnotationsT> KeyValuePairIteratorForBTreeMap<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(inner: btree_map::Iter<'own, Value<AnnotationsT>, Value<AnnotationsT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'own BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own, AnnotationsT> KeyValuePairIterator<AnnotationsT> for KeyValuePairIteratorForBTreeMap<'own, AnnotationsT> {
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Value<AnnotationsT>, &'own Value<AnnotationsT>)>,
        (MalformedError<AnnotationsT>, &Value<AnnotationsT>),
    > {
        Ok(self.inner.next())
    }
}

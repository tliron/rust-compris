use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForHashMap
//

/// A [KeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForHashMap<'own, AnnotationsT> {
    /// Inner iterator.
    pub inner: hash_map::Iter<'own, Value<AnnotationsT>, Value<AnnotationsT>>,
}

impl<'own, AnnotationsT> KeyValuePairIteratorForHashMap<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(inner: hash_map::Iter<'own, Value<AnnotationsT>, Value<AnnotationsT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'own HashMap<Value<AnnotationsT>, Value<AnnotationsT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own, AnnotationsT> KeyValuePairIterator<AnnotationsT> for KeyValuePairIteratorForHashMap<'own, AnnotationsT> {
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Value<AnnotationsT>, &'own Value<AnnotationsT>)>,
        (MalformedError<AnnotationsT>, &Value<AnnotationsT>),
    > {
        Ok(self.inner.next())
    }
}

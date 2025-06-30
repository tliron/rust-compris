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
    pub inner: hash_map::Iter<'own, Value<AnnotatedT>, Value<AnnotatedT>>,
}

impl<'own, AnnotatedT> KeyValuePairIteratorForHashMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: hash_map::Iter<'own, Value<AnnotatedT>, Value<AnnotatedT>>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for(map: &'own HashMap<Value<AnnotatedT>, Value<AnnotatedT>>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own, AnnotatedT> KeyValuePairIterator<AnnotatedT> for KeyValuePairIteratorForHashMap<'own, AnnotatedT> {
    fn next(
        &mut self,
    ) -> Result<
        Option<(&'own Value<AnnotatedT>, &'own Value<AnnotatedT>)>,
        (MalformedError<AnnotatedT>, &Value<AnnotatedT>),
    > {
        Ok(self.inner.next())
    }
}

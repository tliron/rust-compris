use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForBTreeMap
//

/// A [KeyValuePairIterator] for [BTreeMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForBTreeMap<'own> {
    /// BTreeMap iterator.
    iterator: btree_map::Iter<'own, Value, Value>,
}

impl<'own> KeyValuePairIteratorForBTreeMap<'own> {
    /// Constructor.
    pub fn new(iterator: btree_map::Iter<'own, Value, Value>) -> Self {
        Self { iterator }
    }

    /// Constructor.
    pub fn new_for(map: &'own BTreeMap<Value, Value>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own> KeyValuePairIterator for KeyValuePairIteratorForBTreeMap<'own> {
    fn next(&mut self) -> Result<Option<(&'own Value, &'own Value)>, (MalformedError, &Value)> {
        Ok(self.iterator.next())
    }
}

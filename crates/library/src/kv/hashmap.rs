use super::{super::normal::*, iterator::*};

use std::collections::*;

//
// KeyValuePairIteratorForHashMap
//

/// A [KeyValuePairIterator] for [HashMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForHashMap<'own> {
    /// HashMap iterator.
    iterator: hash_map::Iter<'own, Value, Value>,
}

impl<'own> KeyValuePairIteratorForHashMap<'own> {
    /// Constructor.
    pub fn new(iterator: hash_map::Iter<'own, Value, Value>) -> Self {
        Self { iterator }
    }

    /// Constructor.
    pub fn new_for(map: &'own HashMap<Value, Value>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own> KeyValuePairIterator for KeyValuePairIteratorForHashMap<'own> {
    fn next(&mut self) -> Result<Option<(&'own Value, &'own Value)>, (MalformedError, &Value)> {
        match self.iterator.next() {
            Some(pair) => Ok(Some(pair)),
            None => Ok(None),
        }
    }
}

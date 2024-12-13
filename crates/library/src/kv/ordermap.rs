use super::{super::normal::*, iterator::*};

use ordermap::*;

//
// KeyValuePairIteratorForOrderMap
//

/// A [KeyValuePairIterator] for [OrderMap].
///
/// It's just a simple wrapper.
pub struct KeyValuePairIteratorForOrderMap<'own> {
    /// OrderMap iterator.
    iterator: map::Iter<'own, Value, Value>,
}

impl<'own> KeyValuePairIteratorForOrderMap<'own> {
    /// Constructor.
    pub fn new(iterator: map::Iter<'own, Value, Value>) -> Self {
        Self { iterator }
    }

    /// Constructor.
    pub fn new_for(map: &'own OrderMap<Value, Value>) -> Self {
        Self::new(map.into_iter())
    }
}

impl<'own> KeyValuePairIterator for KeyValuePairIteratorForOrderMap<'own> {
    fn next(&mut self) -> Result<Option<(&'own Value, &'own Value)>, (MalformedError, &Value)> {
        match self.iterator.next() {
            Some(pair) => Ok(Some(pair)),
            None => Ok(None),
        }
    }
}

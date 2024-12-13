use super::value::*;

use std::slice;

//
// ValueIterator
//

/// If the value is a [List](super::list::List), iterates its items. Otherwise just iterates itself once.
pub enum ValueIterator<'own> {
    /// Iterator.
    Iterator(slice::Iter<'own, Value>),

    /// Value.
    Value(&'own Value, bool),
}

impl<'own> ValueIterator<'own> {
    /// Constructor.
    pub fn new(value: &'own Value) -> Self {
        match value {
            Value::List(list) => Self::Iterator(list.value.iter()),
            _ => Self::Value(value, false),
        }
    }
}

impl<'own> Iterator for ValueIterator<'own> {
    type Item = &'own Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Iterator(iter) => iter.next(),

            Self::Value(value, iterated) => {
                if *iterated {
                    None
                } else {
                    *iterated = true;
                    Some(value)
                }
            }
        }
    }
}

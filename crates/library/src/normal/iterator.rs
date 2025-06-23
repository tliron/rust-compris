use super::value::*;

use std::slice;

//
// ValueIterator
//

/// If the value is a [List](super::list::List), iterates its items. Otherwise just iterates itself once.
pub enum ValueIterator<'own, AnnotationsT> {
    /// Iterator.
    Iterator(slice::Iter<'own, Value<AnnotationsT>>),

    /// Value.
    Value(&'own Value<AnnotationsT>, bool),
}

impl<'own, AnnotationsT> ValueIterator<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotationsT>) -> Self {
        match value {
            Value::List(list) => Self::Iterator(list.value.iter()),
            _ => Self::Value(value, false),
        }
    }
}

impl<'own, AnnotationsT> Iterator for ValueIterator<'own, AnnotationsT> {
    type Item = &'own Value<AnnotationsT>;

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

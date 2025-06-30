use super::value::*;

use std::slice;

//
// ValueIterator
//

/// If the value is a [List](super::list::List), iterates its items. Otherwise just iterates itself once.
pub enum ValueIterator<'own, AnnotatedT> {
    /// Iterator.
    Iterator(slice::Iter<'own, Value<AnnotatedT>>),

    /// Value.
    Value(&'own Value<AnnotatedT>, bool),
}

impl<'own, AnnotatedT> ValueIterator<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotatedT>) -> Self {
        match value {
            Value::List(list) => Self::Iterator(list.inner.iter()),
            _ => Self::Value(value, false),
        }
    }
}

impl<'own, AnnotatedT> Iterator for ValueIterator<'own, AnnotatedT> {
    type Item = &'own Value<AnnotatedT>;

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

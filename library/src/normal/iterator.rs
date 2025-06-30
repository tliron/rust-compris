use super::variant::*;

use std::slice;

//
// VariantIterator
//

/// If the variant is a [List](super::list::List), iterates its items. Otherwise just iterates
/// itself once.
pub enum VariantIterator<'own, AnnotatedT> {
    /// Iterator.
    Iterator(slice::Iter<'own, Variant<AnnotatedT>>),

    /// Variant.
    Variant(&'own Variant<AnnotatedT>, bool),
}

impl<'own, AnnotatedT> VariantIterator<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(variant: &'own Variant<AnnotatedT>) -> Self {
        match variant {
            Variant::List(list) => Self::Iterator(list.inner.iter()),
            _ => Self::Variant(variant, false),
        }
    }
}

impl<'own, AnnotatedT> Iterator for VariantIterator<'own, AnnotatedT> {
    type Item = &'own Variant<AnnotatedT>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Iterator(iter) => iter.next(),

            Self::Variant(variant, iterated) => {
                if *iterated {
                    None
                } else {
                    *iterated = true;
                    Some(variant)
                }
            }
        }
    }
}

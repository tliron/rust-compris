use {
    super::{super::annotate::*, depict::*, map::*, variant::*},
    crate::impl_normal,
};

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{
        fmt::{self, Write},
        io, slice, vec,
    },
};

//
// List
//

impl_normal! {
    /// Normal list variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    List(Vec<Variant<AnnotatedT>>)
}

impl<AnnotatedT> List<AnnotatedT> {
    /// Constructor.
    pub fn new_with_capacity(capacity: usize) -> Self
    where
        AnnotatedT: Default,
    {
        Self::from(Vec::with_capacity(capacity))
    }

    /// Push.
    pub fn into_push<ItemT>(&mut self, item: ItemT)
    where
        ItemT: Into<Variant<AnnotatedT>>,
    {
        self.inner.push(item.into());
    }

    /// Push a clone of the item only if the list doesn't contain it.
    /// Return true if successful.
    ///
    /// Useful for treating the list like a set (though it's an inefficient one).
    pub fn push_unique_clone(&mut self, item: &Variant<AnnotatedT>) -> bool
    where
        AnnotatedT: Clone,
    {
        if self.inner.contains(item) {
            false
        } else {
            self.inner.push(item.clone());
            true
        }
    }

    /// Remove an item from the list.
    pub fn remove(&mut self, index: usize) -> Option<Variant<AnnotatedT>> {
        if index < self.inner.len() { Some(self.inner.remove(index)) } else { None }
    }

    /// If the list has a length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn to_pair(&self) -> Option<(&Variant<AnnotatedT>, &Variant<AnnotatedT>)> {
        match self.inner.len() {
            2 => {
                let mut iterator = self.inner.iter();
                Some((iterator.next().expect("first"), iterator.next().expect("second")))
            }
            _ => None,
        }
    }

    /// If the list has a length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn into_pair(self) -> Option<(Variant<AnnotatedT>, Variant<AnnotatedT>)> {
        match self.inner.len() {
            2 => {
                let mut iterator = self.inner.into_iter();
                Some((iterator.next().expect("first"), iterator.next().expect("second")))
            }
            _ => None,
        }
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> List<WithoutAnnotations> {
        self.inner.into_iter().map(|item| item.without_annotations()).collect()
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> List<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        let new_list: List<NewAnnotationsT> = self.inner.into_iter().map(|item| item.into_annotated()).collect();
        if AnnotatedT::can_have_annotations()
            && NewAnnotationsT::can_have_annotations()
            && let Some(annotations) = self.annotated.annotations()
        {
            new_list.with_annotations(annotations.clone())
        } else {
            new_list
        }
    }

    /// [Depict] with [Annotations].
    pub fn annotated_depict(&self, mode: AnnotatedDepictionMode) -> AnnotatedDepictList<'_, AnnotatedT> {
        AnnotatedDepictList::new(self, mode)
    }
}

impl<AnnotatedT> Depict for List<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::depict_list(self.inner.iter(), None, writer, context)
    }
}

impl<AnnotatedT> fmt::Display for List<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('[')?;

        for (item, last) in IterateWithLast::new(self) {
            fmt::Display::fmt(item, formatter)?;
            if !last {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char(']')
    }
}

impl<AnnotatedT> IntoIterator for List<AnnotatedT> {
    type Item = Variant<AnnotatedT>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'own, AnnotatedT> IntoIterator for &'own List<AnnotatedT> {
    type Item = &'own Variant<AnnotatedT>;
    type IntoIter = slice::Iter<'own, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'own, AnnotatedT> IntoIterator for &'own mut List<AnnotatedT> {
    type Item = &'own mut Variant<AnnotatedT>;
    type IntoIter = slice::IterMut<'own, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

// Conversions

impl<AnnotatedT> FromIterator<Variant<AnnotatedT>> for List<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Variant<AnnotatedT>>,
    {
        Vec::from_iter(iterator).into()
    }
}

impl<AnnotatedT> From<Map<AnnotatedT>> for List<AnnotatedT>
where
    AnnotatedT: Clone + Default,
{
    /// List where all items are themselves lists of length 2 (key-value pairs).
    fn from(map: Map<AnnotatedT>) -> Self {
        map.into_iter().map(|(key, value)| vec![key.clone(), value.clone()].into()).collect()
    }
}

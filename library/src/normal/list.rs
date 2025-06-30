use {
    super::{super::annotate::*, debug::*, map::*, variant::*},
    crate::impl_normal,
};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
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
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        AnnotatedT: Default,
        IterableT: IntoIterator<Item = Variant<AnnotatedT>>,
    {
        Self::new(Vec::from_iter(iterable))
    }

    /// Constructor.
    pub fn new_from_clone<'own, IterableT>(iterable: IterableT) -> Self
    where
        AnnotatedT: 'own + Clone + Default,
        IterableT: IntoIterator<Item = &'own Variant<AnnotatedT>>,
    {
        let mut list = Self::default();
        for item in iterable {
            list.inner.push(item.clone());
        }
        list
    }

    /// Constructor.
    pub fn new_with_capacity(capacity: usize) -> Self
    where
        AnnotatedT: Default,
    {
        Self::new(Vec::with_capacity(capacity))
    }

    /// Push a clone of the value only if the list doesn't contain it.
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

    /// If the list has a length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn to_pair(&self) -> Option<(&Variant<AnnotatedT>, &Variant<AnnotatedT>)> {
        match self.inner.len() {
            2 => {
                let mut iterator = self.inner.iter();
                Some((iterator.next().expect("non-empty"), iterator.next().expect("non-empty")))
            }
            _ => None,
        }
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> List<WithoutAnnotations> {
        let new_list: Vec<_> = self.inner.into_iter().map(|item| item.without_annotations()).collect();
        new_list.into()
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> List<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        let vector: Vec<_> = self.inner.into_iter().map(|item| item.into_annotated()).collect();
        let new_list = List::new(vector);
        if AnnotatedT::has_annotations()
            && NewAnnotationsT::has_annotations()
            && let Some(annotations) = self.annotated.get_annotations()
        {
            new_list.with_annotations(annotations.clone())
        } else {
            new_list
        }
    }

    /// [Debuggable] with [Annotations].
    pub fn annotated_debuggable(&self, mode: AnnotatedDebuggableMode) -> AnnotatedDebuggableList<'_, AnnotatedT> {
        AnnotatedDebuggableList::new(self, mode)
    }
}

impl<AnnotatedT> Debuggable for List<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::write_debug_as_list(self.inner.iter(), None, writer, context)
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

impl<AnnotatedT> From<Vec<Variant<AnnotatedT>>> for List<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(vector: Vec<Variant<AnnotatedT>>) -> Self {
        Self::new(vector)
    }
}

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
        let mut list = Self::default();
        for (key, value) in map {
            let entry = Self::new(vec![key.clone(), value.clone()]);
            list.inner.push(entry.into());
        }
        list
    }
}

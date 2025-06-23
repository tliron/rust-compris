use {
    super::{super::annotation::*, debug::*, map::*, value::*},
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
    /// Normal list value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    List(Vec<Value<AnnotationsT>>)
}

impl<AnnotationsT> List<AnnotationsT> {
    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        AnnotationsT: Default,
        IterableT: IntoIterator<Item = Value<AnnotationsT>>,
    {
        Self::new(Vec::from_iter(iterable))
    }

    /// Constructor.
    pub fn new_from_clone<'own, IterableT>(iterable: IterableT) -> Self
    where
        AnnotationsT: 'own + Clone + Default,
        IterableT: IntoIterator<Item = &'own Value<AnnotationsT>>,
    {
        let mut list = Self::default();
        for item in iterable {
            list.value.push(item.clone());
        }
        list
    }

    /// Constructor.
    pub fn new_with_capacity(capacity: usize) -> Self
    where
        AnnotationsT: Default,
    {
        Self::new(Vec::with_capacity(capacity))
    }

    /// Push a clone of the value only if the list doesn't contain it.
    /// Return true if successful.
    ///
    /// Useful for treating the list like a set (though it's an inefficient one).
    pub fn push_unique_clone(&mut self, item: &Value<AnnotationsT>) -> bool
    where
        AnnotationsT: Clone,
    {
        if self.value.contains(item) {
            false
        } else {
            self.value.push(item.clone());
            true
        }
    }

    /// If the list has a length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn to_pair(&self) -> Option<(&Value<AnnotationsT>, &Value<AnnotationsT>)> {
        match self.value.len() {
            2 => {
                let mut iterator = self.value.iter();
                Some((iterator.next().expect("non-empty"), iterator.next().expect("non-empty")))
            }
            _ => None,
        }
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> List<WithoutAnnotations> {
        let new_list: Vec<_> = self.value.into_iter().map(|value| value.without_annotations()).collect();
        new_list.into()
    }

    /// Into different annotations.
    pub fn into_annotated<NewAnnotationsT>(self) -> List<NewAnnotationsT>
    where
        AnnotationsT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        let vector: Vec<_> = self.value.into_iter().map(|value| value.into_annotated()).collect();
        let new_list = List::new(vector);
        if AnnotationsT::is_annotated()
            && NewAnnotationsT::is_annotated()
            && let Some(annotations) = self.annotations.get_annotations()
        {
            new_list.with_annotations(annotations.clone())
        } else {
            new_list
        }
    }

    /// [Debuggable] with [Annotations].
    pub fn annotated_debuggable(&self, mode: AnnotatedDebuggableMode) -> AnnotatedDebuggableList<AnnotationsT> {
        AnnotatedDebuggableList::new(self, mode)
    }
}

impl<AnnotationsT> Debuggable for List<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        utils::write_debug_as_list(self.value.iter(), None, writer, context)
    }
}

impl<AnnotationsT> fmt::Display for List<AnnotationsT> {
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

impl<AnnotationsT> IntoIterator for List<AnnotationsT> {
    type Item = Value<AnnotationsT>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'own, AnnotationsT> IntoIterator for &'own List<AnnotationsT> {
    type Item = &'own Value<AnnotationsT>;
    type IntoIter = slice::Iter<'own, Value<AnnotationsT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'own, AnnotationsT> IntoIterator for &'own mut List<AnnotationsT> {
    type Item = &'own mut Value<AnnotationsT>;
    type IntoIter = slice::IterMut<'own, Value<AnnotationsT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl<AnnotationsT> From<Vec<Value<AnnotationsT>>> for List<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(vector: Vec<Value<AnnotationsT>>) -> Self {
        Self::new(vector)
    }
}

impl<AnnotationsT> FromIterator<Value<AnnotationsT>> for List<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Value<AnnotationsT>>,
    {
        Vec::from_iter(iterator).into()
    }
}

impl<AnnotationsT> From<List<AnnotationsT>> for Vec<Value<AnnotationsT>> {
    fn from(list: List<AnnotationsT>) -> Self {
        list.value
    }
}

impl<'own, AnnotationsT> From<&'own List<AnnotationsT>> for &'own Vec<Value<AnnotationsT>> {
    fn from(list: &'own List<AnnotationsT>) -> Self {
        &list.value
    }
}

impl<AnnotationsT> From<Map<AnnotationsT>> for List<AnnotationsT>
where
    AnnotationsT: Clone + Default,
{
    /// List where all items are themselves lists of length 2 (key-value pairs).
    fn from(map: Map<AnnotationsT>) -> Self {
        let mut list = Self::default();
        for (key, value) in map {
            let entry = Self::new(vec![key.clone(), value.clone()]);
            list.value.push(entry.into());
        }
        list
    }
}

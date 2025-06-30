use {
    super::{
        super::{annotate::*, kv::*},
        debug::*,
        errors::*,
        list::*,
        variant::*,
    },
    crate::impl_normal,
};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        collections::*,
        fmt::{self, Write},
        io,
    },
};

//
// Map
//

impl_normal! {
    /// Normal map variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    Map(BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>)
}

impl<AnnotatedT> Map<AnnotatedT> {
    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        AnnotatedT: Default,
        IterableT: IntoIterator<Item = (Variant<AnnotatedT>, Variant<AnnotatedT>)>,
    {
        Self::new(BTreeMap::from_iter(iterable))
    }

    /// Get.
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
    {
        self.inner.get(&key.into())
    }

    /// Insert.
    pub fn into_insert<KeyT, ValueT>(&mut self, key: KeyT, value: ValueT) -> Option<Variant<AnnotatedT>>
    where
        KeyT: Into<Variant<AnnotatedT>>,
        ValueT: Into<Variant<AnnotatedT>>,
    {
        self.inner.insert(key.into(), value.into())
    }

    /// True if any of the map keys is a collection.
    pub fn has_a_collection_key(&self) -> bool {
        for key in self.inner.keys() {
            if key.is_collection() {
                return true;
            }
        }
        false
    }

    /// If the map has *only* one key then returns the key-value tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Variant<AnnotatedT>, &Variant<AnnotatedT>)> {
        match self.inner.len() {
            1 => return self.inner.iter().next(),
            _ => None,
        }
    }

    /// Removes all entries from the map and returns them as a vector of key-value tuples.
    pub fn into_vector(&mut self) -> Vec<(Variant<AnnotatedT>, Variant<AnnotatedT>)> {
        let mut vector = Vec::with_capacity(self.inner.len());
        while let Some(entry) = self.inner.pop_first() {
            vector.push(entry);
        }
        vector
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> Map<WithoutAnnotations> {
        let new_map: BTreeMap<_, _> = self
            .inner
            .into_iter()
            .map(|(key, value)| (key.without_annotations(), value.without_annotations()))
            .collect();
        new_map.into()
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(mut self) -> Map<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        let vector: Vec<_> =
            self.into_vector().into_iter().map(|(key, value)| (key.into_annotated(), value.into_annotated())).collect();
        let new_map = Map::from_iter(vector);
        if AnnotatedT::has_annotations()
            && NewAnnotationsT::has_annotations()
            && let Some(annotations) = self.annotated.get_annotations()
        {
            new_map.with_annotations(annotations.clone())
        } else {
            new_map
        }
    }

    /// [Debuggable] with [Annotations].
    pub fn annotated_debuggable(&self, mode: AnnotatedDebuggableMode) -> AnnotatedDebuggableMap<'_, AnnotatedT> {
        AnnotatedDebuggableMap::new(self, mode)
    }
}

impl<AnnotatedT> Debuggable for Map<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        // Upgrade reduced to verbose if there are collection keys
        let override_format = if (context.format == DebugFormat::Reduced) && self.has_a_collection_key() {
            Some(DebugFormat::Verbose)
        } else {
            None
        };

        utils::write_debug_as_map(self.inner.iter(), override_format, writer, context)
    }
}

impl<AnnotatedT> fmt::Display for Map<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('{')?;

        for ((key, value), last) in IterateWithLast::new(self) {
            fmt::Display::fmt(key, formatter)?;
            formatter.write_char(':')?;
            fmt::Display::fmt(value, formatter)?;
            if !last {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char('}')
    }
}

impl<AnnotatedT> IntoIterator for Map<AnnotatedT> {
    type Item = (Variant<AnnotatedT>, Variant<AnnotatedT>);
    type IntoIter = btree_map::IntoIter<Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'own, AnnotatedT> IntoIterator for &'own Map<AnnotatedT> {
    type Item = (&'own Variant<AnnotatedT>, &'own Variant<AnnotatedT>);
    type IntoIter = btree_map::Iter<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'own, AnnotatedT> IntoIterator for &'own mut Map<AnnotatedT> {
    type Item = (&'own Variant<AnnotatedT>, &'own mut Variant<AnnotatedT>);
    type IntoIter = btree_map::IterMut<'own, Variant<AnnotatedT>, Variant<AnnotatedT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

// Conversions

impl<AnnotatedT> From<BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>> for Map<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(map: BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>) -> Self {
        Self::new(map)
    }
}

impl<const SIZE: usize, AnnotatedT> From<[(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]> for Map<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(array: [(Variant<AnnotatedT>, Variant<AnnotatedT>); SIZE]) -> Self {
        BTreeMap::from(array).into()
    }
}

impl<AnnotatedT> FromIterator<(Variant<AnnotatedT>, Variant<AnnotatedT>)> for Map<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Variant<AnnotatedT>, Variant<AnnotatedT>)>,
    {
        BTreeMap::from_iter(iterator).into()
    }
}

impl<AnnotatedT> TryFrom<List<AnnotatedT>> for Map<AnnotatedT>
where
    AnnotatedT: Clone + Default,
{
    type Error = MalformedError<AnnotatedT>;

    /// The iterated values are expected to be [List] of length 2 (key-value pairs).
    ///
    /// Will return an error if it encounters a duplicate key.
    fn try_from(list: List<AnnotatedT>) -> Result<Self, Self::Error> {
        let mut map = Self::default();

        // Repeat until we get a non-error
        let mut iterator = KeyValuePairIteratorForVariantIterator::new_for(&list);
        loop {
            match iterator.next().map_err(|(error, _value)| error)? {
                Some((key, value)) => {
                    map.inner.insert(key.clone(), value.clone());
                }

                None => break,
            }
        }

        Ok(map)
    }
}

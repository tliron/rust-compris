use {
    super::{
        super::{annotation::*, kv::*},
        debug::*,
        errors::*,
        list::*,
        value::*,
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
    /// Normal map value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    Map(BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>)
}

impl<AnnotationsT> Map<AnnotationsT> {
    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        AnnotationsT: Default,
        IterableT: IntoIterator<Item = (Value<AnnotationsT>, Value<AnnotationsT>)>,
    {
        Self::new(BTreeMap::from_iter(iterable))
    }

    /// True if any of the map keys is a collection.
    pub fn has_a_collection_key(&self) -> bool {
        for key in self.value.keys() {
            if key.is_collection() {
                return true;
            }
        }
        false
    }

    /// If the map has *only* one key then returns the key-value tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Value<AnnotationsT>, &Value<AnnotationsT>)> {
        match self.value.len() {
            1 => return self.value.iter().next(),
            _ => None,
        }
    }

    /// Removes all entries from the map and returns them as a vector of key-value tuples.
    pub fn into_vector(&mut self) -> Vec<(Value<AnnotationsT>, Value<AnnotationsT>)> {
        let mut vector = Vec::with_capacity(self.value.len());
        while let Some(entry) = self.value.pop_first() {
            vector.push(entry);
        }
        vector
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> Map<WithoutAnnotations> {
        let new_map: BTreeMap<_, _> = self
            .value
            .into_iter()
            .map(|(key, value)| (key.without_annotations(), value.without_annotations()))
            .collect();
        new_map.into()
    }

    /// Into different annotations.
    pub fn into_annotated<NewAnnotationsT>(mut self) -> Map<NewAnnotationsT>
    where
        AnnotationsT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        let vector: Vec<_> =
            self.into_vector().into_iter().map(|(key, value)| (key.into_annotated(), value.into_annotated())).collect();
        let new_map = Map::from_iter(vector);
        if AnnotationsT::is_annotated()
            && NewAnnotationsT::is_annotated()
            && let Some(annotations) = self.annotations.get_annotations()
        {
            new_map.with_annotations(annotations.clone())
        } else {
            new_map
        }
    }

    /// [Debuggable] with [Annotations].
    pub fn annotated_debuggable(&self, mode: AnnotatedDebuggableMode) -> AnnotatedDebuggableMap<AnnotationsT> {
        AnnotatedDebuggableMap::new(self, mode)
    }
}

impl<AnnotationsT> Debuggable for Map<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        // Upgrade reduced to verbose if there are collection keys
        let override_format = if matches!(context.format, DebugFormat::Reduced) && self.has_a_collection_key() {
            Some(DebugFormat::Verbose)
        } else {
            None
        };

        utils::write_debug_as_map(self.value.iter(), override_format, writer, context)
    }
}

impl<AnnotationsT> fmt::Display for Map<AnnotationsT> {
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

impl<AnnotationsT> IntoIterator for Map<AnnotationsT> {
    type Item = (Value<AnnotationsT>, Value<AnnotationsT>);
    type IntoIter = btree_map::IntoIter<Value<AnnotationsT>, Value<AnnotationsT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'own, AnnotationsT> IntoIterator for &'own Map<AnnotationsT> {
    type Item = (&'own Value<AnnotationsT>, &'own Value<AnnotationsT>);
    type IntoIter = btree_map::Iter<'own, Value<AnnotationsT>, Value<AnnotationsT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'own, AnnotationsT> IntoIterator for &'own mut Map<AnnotationsT> {
    type Item = (&'own Value<AnnotationsT>, &'own mut Value<AnnotationsT>);
    type IntoIter = btree_map::IterMut<'own, Value<AnnotationsT>, Value<AnnotationsT>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl<AnnotationsT> From<BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>> for Map<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(map: BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>>) -> Self {
        Self::new(map)
    }
}

impl<AnnotationsT> From<Map<AnnotationsT>> for BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>> {
    fn from(map: Map<AnnotationsT>) -> Self {
        map.value
    }
}

impl<const SIZE: usize, AnnotationsT> From<[(Value<AnnotationsT>, Value<AnnotationsT>); SIZE]> for Map<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(array: [(Value<AnnotationsT>, Value<AnnotationsT>); SIZE]) -> Self {
        BTreeMap::from(array).into()
    }
}

impl<AnnotationsT> FromIterator<(Value<AnnotationsT>, Value<AnnotationsT>)> for Map<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from_iter<IntoIteratorT>(iterator: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Value<AnnotationsT>, Value<AnnotationsT>)>,
    {
        BTreeMap::from_iter(iterator).into()
    }
}

impl<'own, AnnotationsT> From<&'own Map<AnnotationsT>> for &'own BTreeMap<Value<AnnotationsT>, Value<AnnotationsT>> {
    fn from(map: &'own Map<AnnotationsT>) -> Self {
        &map.value
    }
}

impl<AnnotationsT> TryFrom<List<AnnotationsT>> for Map<AnnotationsT>
where
    AnnotationsT: Clone + Default,
{
    type Error = MalformedError<AnnotationsT>;

    /// The iterated values are expected to be [List] of length 2 (key-value pairs).
    ///
    /// Will return an error if it encounters a duplicate key.
    fn try_from(list: List<AnnotationsT>) -> Result<Self, Self::Error> {
        let mut map = Self::default();

        // Repeat until we get a non-error
        let mut iterator = KeyValuePairIteratorForValueIterator::new_for(&list);
        loop {
            match iterator.next().map_err(|(error, _value)| error)? {
                Some((key, value)) => {
                    map.value.insert(key.clone(), value.clone());
                }

                None => break,
            }
        }

        Ok(map)
    }
}

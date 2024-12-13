use super::{
    super::{kv::*, meta::*},
    errors::*,
    list::*,
    value::*,
};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        cmp::*,
        collections::*,
        fmt::{self, Write},
        hash::*,
        io,
    },
};

//
// Map
//

/// Normal map value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Map {
    /// Actual value.
    ///
    /// Note that we chose [BTreeMap] for our implementation in order to allow maps
    /// to be used in complex keys. By contrast, [HashMap] does not support [Hash]
    /// and does not have deterministic order.
    pub value: BTreeMap<Value, Value>,

    /// Metadata.
    pub meta: Meta,
}

impl Map {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with<MapT>(map: MapT) -> Self
    where
        MapT: Into<BTreeMap<Value, Value>>,
    {
        Self { value: map.into(), ..Default::default() }
    }

    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<Item = (Value, Value)>,
    {
        Self::new_with(BTreeMap::from_iter(iterable))
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

    /// If the map has *only* one key, returns the key-value
    /// tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Value, &Value)> {
        match self.value.len() {
            1 => return self.value.iter().next(),
            _ => None,
        }
    }
}

impl HasMeta for Map {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Map {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
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

impl fmt::Display for Map {
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

// Delegated

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Map {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

impl IntoIterator for Map {
    type Item = (Value, Value);
    type IntoIter = btree_map::IntoIter<Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'own> IntoIterator for &'own Map {
    type Item = (&'own Value, &'own Value);
    type IntoIter = btree_map::Iter<'own, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'own> IntoIterator for &'own mut Map {
    type Item = (&'own Value, &'own mut Value);
    type IntoIter = btree_map::IterMut<'own, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl From<BTreeMap<Value, Value>> for Map {
    fn from(map: BTreeMap<Value, Value>) -> Self {
        Map::new_with(map)
    }
}

impl From<Map> for BTreeMap<Value, Value> {
    fn from(map: Map) -> Self {
        map.value
    }
}

impl<'own> From<&'own Map> for &'own BTreeMap<Value, Value> {
    fn from(map: &'own Map) -> Self {
        &map.value
    }
}

impl TryFrom<List> for Map {
    type Error = MalformedError;

    /// The iterated values are expected to be [List] of length 2 (key-value pairs).
    ///
    /// Will return an error if it encounters a duplicate key.
    fn try_from(list: List) -> Result<Self, Self::Error> {
        let mut map = Self::new();

        // Repeat until we get a non-error
        let mut iterator = KeyValuePairIteratorForValueIterator::new_for(&list);
        loop {
            match iterator.next() {
                Ok(ok) => match ok {
                    Some((key, value)) => {
                        map.value.insert(key.clone(), value.clone());
                    }

                    None => break,
                },

                Err((error, _)) => return Err(error),
            }
        }

        Ok(map)
    }
}

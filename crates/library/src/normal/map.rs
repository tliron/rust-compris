use super::{
    super::{kv::*, meta::*},
    errors::*,
    list::*,
    value::*,
};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    ordermap::{map, *},
    owo_colors::*,
    std::{
        cmp::*,
        fmt::{self, Write},
        hash::*,
        io,
    },
};

//
// Map
//

/// Normal map value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Map {
    /// Actual value.
    ///
    /// Why we chose [OrderMap]:
    ///
    /// 1. [HashMap](std::collections::HashMap) cannot be hashed
    /// 2. [BTreeMap](std::collections::BTreeMap) can be hashed, but does sorting, which we do not want
    /// 3. [OrderMap] can be hashed, and does not do sorting;
    ///    Of course it does retain insertion order, which is actually useful when deterministic results
    ///    are needed (e.g. in testing)
    pub value: OrderMap<Value, Value>,

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
        MapT: Into<OrderMap<Value, Value>>,
    {
        Self { value: map.into(), ..Default::default() }
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
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let child_prefix = prefix.with("  ");

        for ((key, value), first) in IterateWithFirst::new(self) {
            prefix.conditional_write_with(writer, "? ", first)?;
            key.write_debug_representation(writer, &child_prefix, theme)?;

            prefix.write_with(writer, ": ")?;
            value.write_debug_representation(writer, &child_prefix, theme)?;
        }

        Ok(())
    }
}

impl Map {
    /// Compact version of [Debuggable::write_debug_representation].
    pub fn write_compact_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", "{".style(theme.delimiter))?;

        for ((key, value), last) in IterateWithLast::new(self) {
            key.write_compact_debug_representation(writer, theme)?;
            write!(writer, "{}", ":".style(theme.delimiter))?;
            value.write_compact_debug_representation(writer, theme)?;
            if !last {
                write!(writer, "{}", ",".style(theme.delimiter))?;
            }
        }

        write!(writer, "{}", "}".style(theme.delimiter))
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
    type IntoIter = map::IntoIter<Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'own> IntoIterator for &'own Map {
    type Item = (&'own Value, &'own Value);
    type IntoIter = map::Iter<'own, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'own> IntoIterator for &'own mut Map {
    type Item = (&'own Value, &'own mut Value);
    type IntoIter = map::IterMut<'own, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl From<OrderMap<Value, Value>> for Map {
    fn from(map: OrderMap<Value, Value>) -> Self {
        Map::new_with(map)
    }
}

impl<'own> From<&'own Map> for &'own OrderMap<Value, Value> {
    fn from(map: &'own Map) -> Self {
        &map.value
    }
}

impl TryFrom<List> for Map {
    type Error = MalformedError;

    /// The iterated values are expected to be [List] of length 2 (key-value pairs).
    ///
    /// Keeps track of keys and will report errors if it encounters duplicates.
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

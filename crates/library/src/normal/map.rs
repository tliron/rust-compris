use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    ordermap::{map, *},
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
    // Why we chose OrderMap:
    //
    // 1. HashMap cannot be hashed
    // 2. BTreeMap can be hashed, but does sorting, which we do not want
    // 3. OrderMap can be hashed, and does not do sorting;
    //    Of course it does retain insertion order... which might be useful?
    /// Actual value.
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
    pub fn new_with(value: impl Into<OrderMap<Value, Value>>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl Normal for Map {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        let mut buffer = '{'.to_string();
        let entries: Vec<String> =
            self.value.iter().map(|(k, v)| k.to_map_string_key() + ":" + &v.to_map_string_key()).collect();
        buffer.push_str(&entries.join(","));
        buffer.push('}');
        buffer
    }
}

impl Debuggable for Map {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let child_prefix = prefix.with("  ");

        for ((key, value), first) in FirstIterator::new(self) {
            prefix.write_with(writer, "? ", first)?;
            key.write_debug_representation(writer, &child_prefix, styles)?;

            prefix.write_with(writer, ": ", false)?;
            value.write_debug_representation(writer, &child_prefix, styles)?;
        }

        Ok(())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('{')?;

        for ((key, value), last) in LastIterator::new(self) {
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

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a Value, &'a Value);
    type IntoIter = map::Iter<'a, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'a> IntoIterator for &'a mut Map {
    type Item = (&'a Value, &'a mut Value);
    type IntoIter = map::IterMut<'a, Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl TryFrom<List> for Map {
    type Error = IncompatibleValueTypeError;

    /// Only works if all items of the list are themselves lists of length 2 (key-value pairs),
    /// and that no key is repeated.
    fn try_from(list: List) -> Result<Self, Self::Error> {
        let mut map = Self::new();

        for item in &list {
            match item {
                Value::List(list) => {
                    if let Some((map_key, map_value)) = list.to_couple() {
                        if map.value.insert(map_key.clone(), map_value.clone()).is_some() {
                            return Err(IncompatibleValueTypeError::new(item, "list", Some("without repeating keys")));
                        }
                    } else {
                        return Err(IncompatibleValueTypeError::new(item, "list", Some("of length 2")));
                    }
                }

                _ => return Err(IncompatibleValueTypeError::new(item, "list", Some("of lists of length 2"))),
            }
        }

        Ok(map)
    }
}

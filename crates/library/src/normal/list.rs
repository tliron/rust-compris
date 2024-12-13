use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        cmp::*,
        fmt::{self, Write},
        hash::*,
        io, slice, vec,
    },
};

//
// List
//

/// Normal list value.
#[derive(Debug, Default, Clone, Eq)]
pub struct List {
    /// Actual value.
    pub value: Vec<Value>,

    /// Metadata.
    pub meta: Meta,
}

impl List {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with(value: impl Into<Vec<Value>>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }

    /// Push a clone of the value only if the list doesn't contain it.
    pub fn push_unique_clone(&mut self, value: &Value) -> bool {
        if self.value.contains(value) {
            false
        } else {
            self.value.push(value.clone());
            true
        }
    }

    /// If the list has a length of 2, returns it as a tuple.
    pub fn to_couple(&self) -> Option<(&Value, &Value)> {
        if self.value.len() == 2 {
            let mut iterator = self.value.iter();
            Some((iterator.next().unwrap(), iterator.next().unwrap()))
        } else {
            None
        }
    }
}

impl Normal for List {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        let mut buffer = '['.to_string();
        let items: Vec<String> = self.value.iter().map(|e| e.to_map_string_key()).collect();
        buffer.push_str(&items.join(","));
        buffer.push(']');
        buffer
    }
}

impl Debuggable for List {
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

        for (item, first) in FirstIterator::new(self) {
            prefix.write_with(writer, "- ", first)?;
            item.write_debug_representation(writer, &child_prefix, styles)?;
        }

        Ok(())
    }
}

impl fmt::Display for List {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('[')?;

        for (item, last) in LastIterator::new(self) {
            fmt::Display::fmt(item, formatter)?;
            if !last {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char(']')
    }
}

// Delegated

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for List {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Value;
    type IntoIter = slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'a> IntoIterator for &'a mut List {
    type Item = &'a mut Value;
    type IntoIter = slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl From<Map> for List {
    /// List where all items are themselves lists of length 2 (key-value pairs).
    fn from(map: Map) -> Self {
        let mut list = List::new();
        for (key, value) in map {
            let entry = List::new_with(vec![key.clone(), value.clone()]);
            list.value.push(entry.into());
        }
        list
    }
}

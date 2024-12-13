use super::{super::meta::*, map::*, value::*};

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
#[derive(Clone, Debug, Default, Eq)]
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
    pub fn new_with<VectorT>(vector: VectorT) -> Self
    where
        VectorT: Into<Vec<Value>>,
    {
        Self { value: vector.into(), ..Default::default() }
    }

    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<Item = Value>,
    {
        Self::new_with(Vec::from_iter(iterable))
    }

    /// Constructor.
    pub fn new_from_clone<'own, IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<Item = &'own Value>,
    {
        let mut list = Self::new();
        for item in iterable {
            list.value.push(item.clone());
        }
        list
    }

    /// Constructor.
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self { value: Vec::with_capacity(capacity), ..Default::default() }
    }

    /// Push a clone of the value only if the list doesn't contain it.
    /// Return true if successful.
    ///
    /// Useful for treating the list like a set (though it's an inefficient one).
    pub fn push_unique_clone(&mut self, item: &Value) -> bool {
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
    pub fn to_pair(&self) -> Option<(&Value, &Value)> {
        match self.value.len() {
            2 => {
                let mut iterator = self.value.iter();
                Some((iterator.next().unwrap(), iterator.next().unwrap()))
            }
            _ => None,
        }
    }
}

impl HasMeta for List {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for List {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        utils::write_debug_as_list(self.value.iter(), None, writer, context)
    }
}

impl fmt::Display for List {
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
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'own> IntoIterator for &'own List {
    type Item = &'own Value;
    type IntoIter = slice::Iter<'own, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'own> IntoIterator for &'own mut List {
    type Item = &'own mut Value;
    type IntoIter = slice::IterMut<'own, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut()
    }
}

// Conversions

impl From<Vec<Value>> for List {
    fn from(vector: Vec<Value>) -> Self {
        List::new_with(vector)
    }
}

impl From<List> for Vec<Value> {
    fn from(list: List) -> Self {
        list.value
    }
}

impl<'own> From<&'own List> for &'own Vec<Value> {
    fn from(list: &'own List) -> Self {
        &list.value
    }
}

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

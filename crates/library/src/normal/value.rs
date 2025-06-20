use super::{
    super::kv::*, blob::*, boolean::*, float::*, integer::*, iterator::*, list::*, map::*, null::*, text::*,
    unsigned_integer::*,
};

use std::{cmp::*, hash::*, mem::*};

//
// Value
//

/// Container for a normal value.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value {
    /// Signifies no value.
    #[default]
    Nothing,

    /// Null.
    Null(Null),

    /// Integer.
    Integer(Integer),

    /// Unsigned integer.
    UnsignedInteger(UnsignedInteger),

    /// Float.
    Float(Float),

    /// Boolean.
    Boolean(Boolean),

    /// Text.
    Text(Text),

    /// Blob.
    Blob(Blob),

    /// List.
    List(List),

    /// Map.
    Map(Map),
}

impl Value {
    /// The value's type name.
    pub fn get_type_name(&self) -> &'static str {
        match self {
            Self::Nothing => "Nothing",
            Self::Null(_) => "Null",
            Self::Integer(_) => "Integer",
            Self::UnsignedInteger(_) => "UnsignedInteger",
            Self::Float(_) => "Float",
            Self::Boolean(_) => "Boolean",
            Self::Text(_) => "Text",
            Self::Blob(_) => "Blob",
            Self::List(_) => "List",
            Self::Map(_) => "Map",
        }
    }

    /// True if [Nothing](Value::Nothing).
    pub fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }

    /// True if [List] or [Map].
    pub fn is_collection(&self) -> bool {
        matches!(self, Self::List(_) | Self::Map(_))
    }

    /// Gets a reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn get(&self, key: &Self) -> Option<&Self> {
        match self {
            Self::Map(map) => map.value.get(key),

            Self::List(list) => match key {
                Self::UnsignedInteger(unsigned_integer) => list.value.get(unsigned_integer.value as usize),
                Self::Integer(integer) => list.value.get(integer.value as usize),
                _ => None,
            },

            _ => None,
        }
    }

    /// Gets a mutable reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn get_mut(&mut self, key: &Self) -> Option<&mut Self> {
        match self {
            Value::Map(map) => map.value.get_mut(key),

            Self::List(list) => match key {
                Value::UnsignedInteger(unsigned_integer) => list.value.get_mut(unsigned_integer.value as usize),
                Value::Integer(integer) => list.value.get_mut(integer.value as usize),
                _ => None,
            },

            _ => None,
        }
    }

    /// Gets a reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Self>
    where
        KeyT: Into<Self>,
    {
        self.get(&key.into())
    }

    /// Gets a mutable reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn into_get_mut<KeyT>(&mut self, key: KeyT) -> Option<&mut Self>
    where
        KeyT: Into<Self>,
    {
        self.get_mut(&key.into())
    }

    /// Traverse a value by calling [Value::get] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// You can provide a [ValuePath](super::value_path::ValuePath) iterator as the argument.
    ///
    /// Use the [traverse!](crate::traverse) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse<'own, IteratorT>(&self, keys: IteratorT) -> Option<&Self>
    where
        IteratorT: Iterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get(key)?;
        }
        Some(found)
    }

    /// Traverse a value by calling [Value::get_mut] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// You can provide a [ValuePath](super::value_path::ValuePath) iterator as the argument.
    ///
    /// Use the [traverse_mut!](crate::traverse_mut) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse_mut<'own, IteratorT>(&mut self, keys: IteratorT) -> Option<&mut Self>
    where
        IteratorT: Iterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get_mut(key)?;
        }
        Some(found)
    }

    /// Compare type.
    pub fn same_type(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }

    /// If the value is a [List] with length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn to_pair(&self) -> Option<(&Self, &Self)> {
        match self {
            Self::List(list) => list.to_pair(),
            _ => None,
        }
    }

    /// If the value is a [Map] with *only* one key, returns the key-value
    /// tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Self, &Self)> {
        match self {
            Self::Map(map) => map.to_key_value_pair(),
            _ => None,
        }
    }

    /// If the value is a [List], iterates its items. Otherwise just iterates itself once.
    pub fn iterator(&self) -> ValueIterator {
        ValueIterator::new(self)
    }

    /// An iterator for key-value pairs.
    ///
    /// Can be used on a [Map] or a [List]. The items in a [List] are expected to each be
    /// key-value pairs ([List] of length 2) with unique keys.
    ///
    /// Note that the implementation relies on `dyn` to support two different [KeyValuePairIterator]
    /// implementations.
    pub fn key_value_iterator<'own>(&'own self) -> Option<Box<dyn KeyValuePairIterator + 'own>> {
        match self {
            Value::Map(map) => Some(Box::new(KeyValuePairIteratorForBTreeMap::new_for(&map.value))),
            Value::List(list) => Some(Box::new(KeyValuePairIteratorForValueIterator::new_for(list))),
            _ => None,
        }
    }
}

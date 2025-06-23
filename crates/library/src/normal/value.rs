use super::{
    super::{annotation::*, kv::*, path::*},
    blob::*,
    boolean::*,
    debug::*,
    float::*,
    integer::*,
    iterator::*,
    list::*,
    map::*,
    null::*,
    text::*,
    unsigned_integer::*,
};

use {bytestring::*, std::mem::*};

//
// Value
//

/// Container for normal types.
#[derive(Clone, Debug, Default)]
pub enum Value<AnnotationsT> {
    /// Signifies no value.
    #[default]
    Nothing,

    /// Null.
    Null(Null<AnnotationsT>),

    /// Integer.
    Integer(Integer<AnnotationsT>),

    /// Unsigned integer.
    UnsignedInteger(UnsignedInteger<AnnotationsT>),

    /// Float.
    Float(Float<AnnotationsT>),

    /// Boolean.
    Boolean(Boolean<AnnotationsT>),

    /// Text.
    Text(Text<AnnotationsT>),

    /// Blob.
    Blob(Blob<AnnotationsT>),

    /// List.
    List(List<AnnotationsT>),

    /// Map.
    Map(Map<AnnotationsT>),
}

impl<AnnotationsT> Value<AnnotationsT> {
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

    /// Compare type.
    pub fn same_type(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
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
    /// Use the [traverse!](crate::traverse) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse<'own, IteratorT>(&self, keys: IteratorT) -> Option<&Self>
    where
        AnnotationsT: 'own,
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
    /// Use the [traverse_mut!](crate::traverse_mut) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse_mut<'own, IteratorT>(&mut self, keys: IteratorT) -> Option<&mut Self>
    where
        AnnotationsT: 'own,
        IteratorT: Iterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get_mut(key)?;
        }
        Some(found)
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
    pub fn iterator(&self) -> ValueIterator<AnnotationsT> {
        ValueIterator::new(self)
    }

    /// An iterator for key-value pairs.
    ///
    /// Can be used on a [Map] or a [List]. The items in a [List] are expected to each be
    /// key-value pairs ([List] of length 2) with unique keys.
    ///
    /// Note that the implementation relies on `dyn` to support two different [KeyValuePairIterator]
    /// implementations.
    pub fn key_value_iterator<'own>(&'own self) -> Option<Box<dyn KeyValuePairIterator<AnnotationsT> + 'own>>
    where
        AnnotationsT: Default,
    {
        match self {
            Self::Map(map) => Some(Box::new(KeyValuePairIteratorForBTreeMap::new_for(&map.value))),
            Self::List(list) => Some(Box::new(KeyValuePairIteratorForValueIterator::new_for(list))),
            _ => None,
        }
    }

    /// Removes all [Annotations] recursively.
    pub fn without_annotations(self) -> Value<WithoutAnnotations> {
        match self {
            Self::Nothing => Value::Nothing,
            Self::Null(null) => Value::Null(null.without_annotations()),
            Self::Integer(integer) => Value::Integer(integer.without_annotations()),
            Self::UnsignedInteger(unsigned_integer) => Value::UnsignedInteger(unsigned_integer.without_annotations()),
            Self::Float(float) => Value::Float(float.without_annotations()),
            Self::Boolean(boolean) => Value::Boolean(boolean.without_annotations()),
            Self::Text(text) => Value::Text(text.without_annotations()),
            Self::Blob(blob) => Value::Blob(blob.without_annotations()),
            Self::List(list) => Value::List(list.without_annotations()),
            Self::Map(map) => Value::Map(map.without_annotations()),
        }
    }

    /// Into different annotations.
    pub fn into_annotated<NewAnnotationsT>(self) -> Value<NewAnnotationsT>
    where
        AnnotationsT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        match self {
            Self::Nothing => Value::Nothing,
            Self::Null(null) => Value::Null(null.into_annotated()),
            Self::Integer(integer) => Value::Integer(integer.into_annotated()),
            Self::UnsignedInteger(unsigned_integer) => Value::UnsignedInteger(unsigned_integer.into_annotated()),
            Self::Float(float) => Value::Float(float.into_annotated()),
            Self::Boolean(boolean) => Value::Boolean(boolean.into_annotated()),
            Self::Text(text) => Value::Text(text.into_annotated()),
            Self::Blob(blob) => Value::Blob(blob.into_annotated()),
            Self::List(list) => Value::List(list.into_annotated()),
            Self::Map(map) => Value::Map(map.into_annotated()),
        }
    }

    /// Add source and [PathRepresentation] to all [Annotations] recursively.
    pub fn annotated(mut self, source: &Option<ByteString>) -> Self
    where
        AnnotationsT: Annotated + Default,
    {
        if AnnotationsT::is_annotated() {
            let path = self.get_annotations().and_then(|annotations| annotations.path.clone()).unwrap_or_default();
            self.annotate_with_base_path(source, &path);
        }
        self
    }

    fn annotate_with_base_path(&mut self, source: &Option<ByteString>, base_path: &PathRepresentation)
    where
        AnnotationsT: Annotated + Default,
    {
        if source.is_some() {
            if let Some(annotations) = self.get_annotations_mut() {
                annotations.source = source.clone();
            }
        }

        match self {
            Self::List(list) => {
                for (index, value) in list.value.iter_mut().enumerate() {
                    let mut path = base_path.clone();
                    path.push_list_index(index);
                    value.annotate_with_base_path(source, &path);

                    if let Some(annotations) = value.get_annotations_mut() {
                        annotations.path = Some(path);
                    }
                }
            }

            Self::Map(map) => {
                let mut vector = map.into_vector();

                for (key, value) in vector.iter_mut() {
                    let mut path = base_path.clone();
                    path.push_map_key(key.to_string().into());

                    key.annotate_with_base_path(source, &path);
                    value.annotate_with_base_path(source, &path);

                    if let Some(annotations) = key.get_annotations_mut() {
                        annotations.path = Some(path.clone());
                    }

                    if let Some(annotations) = value.get_annotations_mut() {
                        annotations.path = Some(path);
                    }
                }

                *map = Map::from_iter(vector).with_annotations_from(map);
            }

            _ => {}
        }
    }

    /// [Debuggable](kutil_cli::debug::Debuggable) with [Annotations].
    pub fn annotated_debuggable(&self) -> AnnotatedDebuggableValue<AnnotationsT> {
        AnnotatedDebuggableValue::new(self, AnnotatedDebuggableMode::Inline)
    }
}

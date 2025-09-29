use super::{
    super::{annotate::*, kv::*, path::*},
    blob::*,
    boolean::*,
    depict::*,
    float::*,
    integer::*,
    iterator::*,
    list::*,
    map::*,
    null::*,
    text::*,
    unsigned_integer::*,
};

use {kutil::std::immutable::*, std::mem::*};

//
// Variant
//

/// Container for normal types.
#[derive(Clone, Debug, Default)]
pub enum Variant<AnnotatedT> {
    /// Undefined. This is different from [Null]!
    #[default]
    Undefined,

    /// Null. This is different from [Undefined](Variant::Undefined)!
    Null(Null<AnnotatedT>),

    /// Integer.
    Integer(Integer<AnnotatedT>),

    /// Unsigned integer.
    UnsignedInteger(UnsignedInteger<AnnotatedT>),

    /// Float.
    Float(Float<AnnotatedT>),

    /// Boolean.
    Boolean(Boolean<AnnotatedT>),

    /// Text.
    Text(Text<AnnotatedT>),

    /// Blob.
    Blob(Blob<AnnotatedT>),

    /// List.
    List(List<AnnotatedT>),

    /// Map.
    Map(Map<AnnotatedT>),
}

impl<AnnotatedT> Variant<AnnotatedT> {
    /// The variant's type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Undefined => "Undefined",
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

    /// True if [Undefined](Variant::Undefined).
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// True if [Null].
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null(_))
    }

    /// True if [List] or [Map].
    pub fn is_collection(&self) -> bool {
        matches!(self, Self::List(_) | Self::Map(_))
    }

    /// Gets a reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Variant::UnsignedInteger] or an [Variant::Integer].
    pub fn get(&self, key: &Self) -> Option<&Self> {
        match (self, key) {
            (Self::Map(map), key) => map.inner.get(key),
            (Self::List(list), Self::UnsignedInteger(unsigned_integer)) => {
                list.inner.get(unsigned_integer.inner as usize)
            }
            (Self::List(list), Self::Integer(integer)) => list.inner.get(integer.inner as usize),

            _ => None,
        }
    }

    /// Gets a mutable reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an [UnsignedInteger]
    /// or an [Integer].
    pub fn get_mut(&mut self, key: &Self) -> Option<&mut Self> {
        match (self, key) {
            (Self::Map(map), key) => map.inner.get_mut(key),
            (Self::List(list), Self::UnsignedInteger(unsigned_integer)) => {
                list.inner.get_mut(unsigned_integer.inner as usize)
            }
            (Self::List(list), Self::Integer(integer)) => list.inner.get_mut(integer.inner as usize),

            _ => None,
        }
    }

    /// Removes a reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Variant::UnsignedInteger] or an [Variant::Integer].
    pub fn remove(&mut self, key: &Self) -> Option<Self> {
        match (self, key) {
            (Self::Map(map), key) => map.inner.remove(key),
            (Self::List(list), Self::UnsignedInteger(unsigned_integer)) => list.remove(unsigned_integer.inner as usize),
            (Self::List(list), Self::Integer(integer)) => list.remove(integer.inner as usize),

            _ => None,
        }
    }

    /// Gets a reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an [UnsignedInteger]
    /// or an [Integer].
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Self>
    where
        KeyT: Into<Self>,
    {
        self.get(&key.into())
    }

    /// Gets a mutable reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an [UnsignedInteger]
    /// or an [Integer].
    pub fn into_get_mut<KeyT>(&mut self, key: KeyT) -> Option<&mut Self>
    where
        KeyT: Into<Self>,
    {
        self.get_mut(&key.into())
    }

    /// Removes a reference to a nested variant.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Variant::UnsignedInteger] or an [Variant::Integer].
    pub fn into_remove<KeyT>(&mut self, key: KeyT) -> Option<Self>
    where
        KeyT: Into<Self>,
    {
        self.remove(&key.into())
    }

    /// Traverse the variant by calling [Variant::get] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// Use the [traverse!](crate::traverse) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse<'own, IteratorT>(&self, keys: IteratorT) -> Option<&Self>
    where
        AnnotatedT: 'own,
        IteratorT: Iterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get(key)?;
        }
        Some(found)
    }

    /// Traverse the variant by calling [Variant::get_mut] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// Use the [traverse_mut!](crate::traverse_mut) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated iterator.
    pub fn traverse_mut<'own, IteratorT>(&mut self, keys: IteratorT) -> Option<&mut Self>
    where
        AnnotatedT: 'own,
        IteratorT: Iterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get_mut(key)?;
        }
        Some(found)
    }

    /// If the variant is a [List] with length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn to_pair(&self) -> Option<(&Self, &Self)> {
        match self {
            Self::List(list) => list.to_pair(),
            _ => None,
        }
    }

    /// If the variant is a [List] with length of 2, returns it as a tuple.
    ///
    /// Useful when using the list as a key-value pair for a map.
    pub fn into_pair(self) -> Option<(Self, Self)> {
        match self {
            Self::List(list) => list.into_pair(),
            _ => None,
        }
    }

    /// If the variant is a [Map] with *only* one key, returns the key-value tuple.
    pub fn to_key_value_pair(&self) -> Option<(&Self, &Self)> {
        match self {
            Self::Map(map) => map.to_key_value_pair(),
            _ => None,
        }
    }

    /// If the variant is a [Map] with *only* one key, returns the key-value tuple.
    pub fn into_key_value_pair(self) -> Option<(Self, Self)> {
        match self {
            Self::Map(map) => map.into_key_value_pair(),
            _ => None,
        }
    }

    /// If the variant is a [List], iterates its items. Otherwise just iterates itself once.
    pub fn iterator(&self) -> VariantIterator<'_, AnnotatedT> {
        VariantIterator::new(self)
    }

    /// If the variant is a [List], iterates its items. Otherwise just iterates itself once.
    pub fn into_iterator(self) -> IntoVariantIterator<AnnotatedT> {
        IntoVariantIterator::new(self)
    }

    /// An iterator for key-value pairs.
    ///
    /// Can be used on a [Map] or a [List]. The items in a [List] are expected to each be key-value
    /// pairs ([List] of length 2) with unique keys.
    ///
    /// Note that the iterator is a `dyn` in order to support different underlying implementations.
    pub fn into_key_value_iterator<'own>(self) -> Option<Box<dyn IntoKeyValuePairIterator<AnnotatedT> + 'own>>
    where
        AnnotatedT: 'own + Clone + Default,
    {
        match self {
            Self::Map(map) => Some(Box::new(IntoKeyValuePairIteratorForBTreeMap::new_for(map.inner))),
            Self::List(list) => Some(Box::new(IntoKeyValuePairIteratorForVariantIterator::new_for(list))),
            _ => None,
        }
    }

    /// An iterator for key-value pairs.
    ///
    /// Can be used on a [Map] or a [List]. The items in a [List] are expected to each be key-value
    /// pairs ([List] of length 2) with unique keys.
    ///
    /// Note that the iterator is a `dyn` in order to support different underlying implementations.
    pub fn key_value_iterator<'own>(&'own self) -> Option<Box<dyn KeyValuePairIterator<AnnotatedT> + 'own>>
    where
        AnnotatedT: Default,
    {
        match self {
            Self::Map(map) => Some(Box::new(KeyValuePairIteratorForBTreeMap::new_for(&map.inner))),
            Self::List(list) => Some(Box::new(KeyValuePairIteratorForVariantIterator::new_for(list))),
            _ => None,
        }
    }

    /// Remove all [Annotations] recursively.
    pub fn without_annotations(self) -> Variant<WithoutAnnotations> {
        match self {
            Self::Undefined => Variant::Undefined,
            Self::Null(null) => Variant::Null(null.without_annotations()),
            Self::Integer(integer) => Variant::Integer(integer.without_annotations()),
            Self::UnsignedInteger(unsigned_integer) => Variant::UnsignedInteger(unsigned_integer.without_annotations()),
            Self::Float(float) => Variant::Float(float.without_annotations()),
            Self::Boolean(boolean) => Variant::Boolean(boolean.without_annotations()),
            Self::Text(text) => Variant::Text(text.without_annotations()),
            Self::Blob(blob) => Variant::Blob(blob.without_annotations()),
            Self::List(list) => Variant::List(list.without_annotations()),
            Self::Map(map) => Variant::Map(map.without_annotations()),
        }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> Variant<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        match self {
            Self::Undefined => Variant::Undefined,
            Self::Null(null) => Variant::Null(null.into_annotated()),
            Self::Integer(integer) => Variant::Integer(integer.into_annotated()),
            Self::UnsignedInteger(unsigned_integer) => Variant::UnsignedInteger(unsigned_integer.into_annotated()),
            Self::Float(float) => Variant::Float(float.into_annotated()),
            Self::Boolean(boolean) => Variant::Boolean(boolean.into_annotated()),
            Self::Text(text) => Variant::Text(text.into_annotated()),
            Self::Blob(blob) => Variant::Blob(blob.into_annotated()),
            Self::List(list) => Variant::List(list.into_annotated()),
            Self::Map(map) => Variant::Map(map.into_annotated()),
        }
    }

    /// Add source and [PathRepresentation] to all [Annotations] recursively.
    pub fn fully_annotated(mut self, source: &Option<ByteString>) -> Self
    where
        AnnotatedT: Annotated + Default,
    {
        if AnnotatedT::can_have_annotations() {
            let path = self.annotations().and_then(|annotations| annotations.path.clone()).unwrap_or_default();
            self.fully_annotate(source, &path);
        }
        self
    }

    fn fully_annotate(&mut self, source: &Option<ByteString>, base_path: &PathRepresentation)
    where
        AnnotatedT: Annotated + Default,
    {
        if source.is_some() {
            if let Some(annotations) = self.annotations_mut() {
                annotations.source = source.clone();
            }
        }

        match self {
            Self::List(list) => {
                for (index, value) in list.inner.iter_mut().enumerate() {
                    let mut path = base_path.clone();
                    path.push_list_index(index);
                    value.fully_annotate(source, &path);

                    if let Some(annotations) = value.annotations_mut() {
                        annotations.path = Some(path);
                    }
                }
            }

            Self::Map(map) => {
                let mut vector = map.into_vector();

                for (key, value) in vector.iter_mut() {
                    let mut path = base_path.clone();
                    path.push_map_key(key.to_string().into());

                    key.fully_annotate(source, &path);
                    value.fully_annotate(source, &path);

                    if let Some(annotations) = key.annotations_mut() {
                        annotations.path = Some(path.clone());
                    }

                    if let Some(annotations) = value.annotations_mut() {
                        annotations.path = Some(path);
                    }
                }

                *map = Map::from_iter(vector).with_annotations_from(map);
            }

            _ => {}
        }
    }

    /// [Depict](kutil::cli::depict::Depict) with [Annotations].
    pub fn annotated_depict(&self) -> AnnotatedDepictVariant<'_, AnnotatedT> {
        AnnotatedDepictVariant::new(self, AnnotatedDepictionMode::Inline)
    }
}

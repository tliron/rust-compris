use super::{super::*, serialization_mode::*};

use {
    serde::ser::*,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

impl Value {
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> ValueWithSerializationMode<'a> {
        ValueWithSerializationMode::new(self, serialization_mode)
    }
}

//
// ValueWithSerializationMode
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueWithSerializationMode<'a> {
    pub value: &'a Value,
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> ValueWithSerializationMode<'a> {
    pub fn new(value: &'a Value, serialization_mode: &'a SerializationMode) -> Self {
        Self { value, serialization_mode }
    }
}

impl<'a> PartialOrd for ValueWithSerializationMode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Ord for ValueWithSerializationMode<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<'a> Hash for ValueWithSerializationMode<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<'a> fmt::Display for ValueWithSerializationMode<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl<'a> WriteDebug for ValueWithSerializationMode<'a> {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        self.value.write_debug_representation(writer, indentation, styles)
    }
}

impl<'a> ToMapStringKey for ValueWithSerializationMode<'a> {
    fn to_map_string_key(&self) -> StdString {
        self.value.to_map_string_key()
    }
}

impl<'a> Serialize for ValueWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.value {
            Value::Empty => Err(Error::custom("empty value")),
            Value::Null(null) => null.serialize(serializer),
            Value::Integer(integer) => integer.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.with_serialization_mode(&self.serialization_mode).serialize(serializer)
            }
            Value::Float(float) => float.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::Boolean(boolean) => boolean.serialize(serializer),
            Value::String(string) => string.serialize(serializer),
            Value::Bytes(bytes) => bytes.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::List(list) => list.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::Map(map) => map.with_serialization_mode(&self.serialization_mode).serialize(serializer),
        }
    }
}

use super::super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// Value
//

impl Value {
    /// Adds [SerializationMode] support.
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> ValueWithSerializationMode<'a> {
        ValueWithSerializationMode::new(self, serialization_mode)
    }
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Nothing => Err(Error::custom("empty value")),
            Value::Null(null) => null.serialize(serializer),
            Value::Integer(integer) => integer.serialize(serializer),
            Value::UnsignedInteger(unsigned_integer) => unsigned_integer.serialize(serializer),
            Value::Float(float) => float.serialize(serializer),
            Value::Boolean(boolean) => boolean.serialize(serializer),
            Value::String(string) => string.serialize(serializer),
            Value::Bytes(bytes) => bytes.serialize(serializer),
            Value::List(list) => list.serialize(serializer),
            Value::Map(map) => map.serialize(serializer),
        }
    }
}

//
// ValueWithSerializationMode
//

/// Adds [SerializationMode] support to [Value].
pub struct ValueWithSerializationMode<'a> {
    /// Wrapped value.
    pub value: &'a Value,

    /// Serialization mode.
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> ValueWithSerializationMode<'a> {
    /// Constructor.
    pub fn new(value: &'a Value, serialization_mode: &'a SerializationMode) -> Self {
        Self { value, serialization_mode }
    }
}

impl<'a> Serialize for ValueWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.value {
            Value::Nothing => Err(Error::custom("empty value")),
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

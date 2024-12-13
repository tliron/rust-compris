use super::super::{super::normal::*, mode::*};

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
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match self {
            Self::Nothing => Err(Error::custom("empty value")),
            Self::Null(null) => null.serialize(serializer),
            Self::Integer(integer) => integer.serialize(serializer),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.serialize(serializer),
            Self::Float(float) => float.serialize(serializer),
            Self::Boolean(boolean) => boolean.serialize(serializer),
            Self::Text(string) => string.serialize(serializer),
            Self::Bytes(bytes) => bytes.serialize(serializer),
            Self::List(list) => list.serialize(serializer),
            Self::Map(map) => map.serialize(serializer),
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
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match &self.value {
            Value::Nothing => Err(Error::custom("empty value")),
            Value::Null(null) => null.serialize(serializer),
            Value::Integer(integer) => integer.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.with_serialization_mode(&self.serialization_mode).serialize(serializer)
            }
            Value::Float(float) => float.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::Boolean(boolean) => boolean.serialize(serializer),
            Value::Text(string) => string.serialize(serializer),
            Value::Bytes(bytes) => bytes.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::List(list) => list.with_serialization_mode(&self.serialization_mode).serialize(serializer),
            Value::Map(map) => map.with_serialization_mode(&self.serialization_mode).serialize(serializer),
        }
    }
}

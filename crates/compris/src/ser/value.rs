use super::super::*;

use serde::ser::*;

//
// Value
//

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Empty => Err(Error::custom("empty value")),
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

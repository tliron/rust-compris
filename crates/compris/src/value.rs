use super::{
    boolean::*, bytes::*, float::*, integer::*, list::*, map::*, null::*, string::*, styles::*, to_map_string_key::*,
    unsigned_integer::*, write_debug::*,
};

use std::{cmp::*, fmt, hash::*, io, string};

//
// Value
//

/// ARD value.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    #[default]
    Empty,
    Null(Null),
    Integer(Integer),
    UnsignedInteger(UnsignedInteger),
    Float(Float),
    Boolean(Boolean),
    String(String),
    Bytes(Bytes),
    List(List),
    Map(Map),
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Empty => write!(formatter, "empty"),
            Value::Null(null) => null.fmt(formatter),
            Value::Integer(integer) => integer.fmt(formatter),
            Value::UnsignedInteger(unsigned_integer) => unsigned_integer.fmt(formatter),
            Value::Float(float) => float.fmt(formatter),
            Value::Boolean(boolean) => boolean.fmt(formatter),
            Value::String(string) => string.fmt(formatter),
            Value::Bytes(bytes) => bytes.fmt(formatter),
            Value::List(list) => list.fmt(formatter),
            Value::Map(map) => map.fmt(formatter),
        }
    }
}

impl WriteDebug for Value {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        match self {
            Value::Empty => write!(writer, "empty"),
            Value::Null(null) => null.write_debug_representation(writer, indentation, styles),
            Value::Integer(integer) => integer.write_debug_representation(writer, indentation, styles),
            Value::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.write_debug_representation(writer, indentation, styles)
            }
            Value::Float(float) => float.write_debug_representation(writer, indentation, styles),
            Value::Boolean(boolean) => boolean.write_debug_representation(writer, indentation, styles),
            Value::String(string) => string.write_debug_representation(writer, indentation, styles),
            Value::Bytes(bytes) => bytes.write_debug_representation(writer, indentation, styles),
            Value::List(list) => list.write_debug_representation(writer, indentation, styles),
            Value::Map(map) => map.write_debug_representation(writer, indentation, styles),
        }
    }
}

impl ToMapStringKey for Value {
    fn to_map_string_key(&self) -> string::String {
        match self {
            Value::Empty => "empty".into(),
            Value::Null(null) => null.to_map_string_key(),
            Value::Integer(integer) => integer.to_map_string_key(),
            Value::UnsignedInteger(unsigned_integer) => unsigned_integer.to_map_string_key(),
            Value::Float(float) => float.to_map_string_key(),
            Value::Boolean(boolean) => boolean.to_map_string_key(),
            Value::String(string) => string.to_map_string_key(),
            Value::Bytes(bytes) => bytes.to_map_string_key(),
            Value::List(list) => list.to_map_string_key(),
            Value::Map(map) => map.to_map_string_key(),
        }
    }
}

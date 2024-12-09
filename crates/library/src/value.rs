use super::{
    boolean::*, bytes::*, float::*, integer::*, list::*, map::*, null::*, string::*, styles::*, to_map_string_key::*,
    unsigned_integer::*, write_debug::*,
};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string},
};

//
// Value
//

/// Container for a normal value.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// Signifies no value.
    #[default]
    Nothing,

    /// Null value.
    Null(Null),

    /// Integer value.
    Integer(Integer),

    /// Unsigned integer value.
    UnsignedInteger(UnsignedInteger),

    /// Float value.
    Float(Float),

    /// Boolean value.
    Boolean(Boolean),

    /// String value.
    String(String),

    /// Bytes value.
    Bytes(Bytes),

    /// List value.
    List(List),

    /// Map value.
    Map(Map),
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(formatter, "nothing"),
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

impl<W: io::Write> WriteDebug<W> for Value {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        match self {
            Value::Nothing => write!(writer, "{}", "nothing".style(styles.plain)),
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
            Value::Nothing => "nothing".into(),
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

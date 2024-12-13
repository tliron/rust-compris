use super::{
    super::*, boolean::*, bytes::*, errors::*, float::*, integer::*, list::*, map::*, normal::*, null::*, text::*,
    unsigned_integer::*,
};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
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

    /// Bytes.
    Bytes(Bytes),

    /// List.
    List(List),

    /// Map.
    Map(Map),
}

impl Value {
    /// The value's type name.
    pub fn get_type_name(&self) -> &'static str {
        match self {
            Self::Nothing => "nothing",
            Self::Null(_) => "null",
            Self::Integer(_) => "integer",
            Self::UnsignedInteger(_) => "unsigned integer",
            Self::Float(_) => "float",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Bytes(_) => "bytes",
            Self::List(_) => "list",
            Self::Map(_) => "map",
        }
    }

    /// Gets a reference to contained value.
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn get(&self, key: impl Into<Self>) -> Option<&Self> {
        let key = key.into();
        match self {
            Self::Map(map) => map.value.get(&key),

            Self::List(list) => match key {
                Value::UnsignedInteger(unsigned_integer) => list.value.get(unsigned_integer.value as usize),
                Value::Integer(integer) => list.value.get(integer.value as usize),
                _ => None,
            },

            _ => None,
        }
    }

    /// Gets a mutable reference to contained value.
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn get_mut(&mut self, key: impl Into<Self>) -> Option<&mut Self> {
        let key = key.into();
        match self {
            Value::Map(map) => map.value.get_mut(&key),

            Self::List(list) => match key {
                Value::UnsignedInteger(unsigned_integer) => list.value.get_mut(unsigned_integer.value as usize),
                Value::Integer(integer) => list.value.get_mut(integer.value as usize),
                _ => None,
            },

            _ => None,
        }
    }
}

impl Normal for Value {
    fn get_meta(&self) -> Option<&Meta> {
        match self {
            Self::Nothing => None,
            Self::Null(null) => null.get_meta(),
            Self::Integer(integer) => integer.get_meta(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_meta(),
            Self::Float(float) => float.get_meta(),
            Self::Boolean(boolean) => boolean.get_meta(),
            Self::Text(text) => text.get_meta(),
            Self::Bytes(bytes) => bytes.get_meta(),
            Self::List(list) => list.get_meta(),
            Self::Map(map) => map.get_meta(),
        }
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        match self {
            Self::Nothing => None,
            Self::Null(null) => null.get_meta_mut(),
            Self::Integer(integer) => integer.get_meta_mut(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_meta_mut(),
            Self::Float(float) => float.get_meta_mut(),
            Self::Boolean(boolean) => boolean.get_meta_mut(),
            Self::Text(text) => text.get_meta_mut(),
            Self::Bytes(bytes) => bytes.get_meta_mut(),
            Self::List(list) => list.get_meta_mut(),
            Self::Map(map) => map.get_meta_mut(),
        }
    }

    fn to_map_string_key(&self) -> String {
        match self {
            Self::Nothing => "nothing".into(),
            Self::Null(null) => null.to_map_string_key(),
            Self::Integer(integer) => integer.to_map_string_key(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.to_map_string_key(),
            Self::Float(float) => float.to_map_string_key(),
            Self::Boolean(boolean) => boolean.to_map_string_key(),
            Self::Text(text) => text.to_map_string_key(),
            Self::Bytes(bytes) => bytes.to_map_string_key(),
            Self::List(list) => list.to_map_string_key(),
            Self::Map(map) => map.to_map_string_key(),
        }
    }
}

impl WriteDebug for Value {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        match self {
            Self::Nothing => write!(writer, "{}", "nothing".style(styles.plain)),
            Self::Null(null) => null.write_debug_representation(writer, indentation, styles),
            Self::Integer(integer) => integer.write_debug_representation(writer, indentation, styles),
            Self::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.write_debug_representation(writer, indentation, styles)
            }
            Self::Float(float) => float.write_debug_representation(writer, indentation, styles),
            Self::Boolean(boolean) => boolean.write_debug_representation(writer, indentation, styles),
            Self::Text(text) => text.write_debug_representation(writer, indentation, styles),
            Self::Bytes(bytes) => bytes.write_debug_representation(writer, indentation, styles),
            Self::List(list) => list.write_debug_representation(writer, indentation, styles),
            Self::Map(map) => map.write_debug_representation(writer, indentation, styles),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(formatter, "nothing"),
            Self::Null(null) => null.fmt(formatter),
            Self::Integer(integer) => integer.fmt(formatter),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.fmt(formatter),
            Self::Float(float) => float.fmt(formatter),
            Self::Boolean(boolean) => boolean.fmt(formatter),
            Self::Text(text) => text.fmt(formatter),
            Self::Bytes(bytes) => bytes.fmt(formatter),
            Self::List(list) => list.fmt(formatter),
            Self::Map(map) => map.fmt(formatter),
        }
    }
}

// From normal types

impl From<Null> for Value {
    fn from(value: Null) -> Self {
        Self::Null(value)
    }
}

impl From<Integer> for Value {
    fn from(value: Integer) -> Self {
        Self::Integer(value)
    }
}

impl From<UnsignedInteger> for Value {
    fn from(value: UnsignedInteger) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<Boolean> for Value {
    fn from(value: Boolean) -> Self {
        Self::Boolean(value)
    }
}

impl From<Text> for Value {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Value::Bytes(value)
    }
}

impl From<List> for Value {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Map> for Value {
    fn from(value: Map) -> Self {
        Self::Map(value)
    }
}

// From primitive types

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Null(Null::new())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::UnsignedInteger(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::UnsignedInteger(value.into())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::UnsignedInteger(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

// To primitive types

impl TryFrom<&Value> for i64 {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(integer) => Ok(integer.value),
            _ => Err(IncompatibleValueTypeError::new(value, "integer")),
        }
    }
}

impl TryFrom<&Value> for u64 {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer.value),
            _ => Err(IncompatibleValueTypeError::new(value, "unsigned integer")),
        }
    }
}

impl TryFrom<&Value> for f64 {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(float) => Ok(float.value.into()),
            _ => Err(IncompatibleValueTypeError::new(value, "float")),
        }
    }
}

impl TryFrom<&Value> for bool {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean.value),
            _ => Err(IncompatibleValueTypeError::new(value, "boolean")),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        // Note: this will clone the string
        match value {
            Value::Text(text) => Ok(text.value.clone()),
            _ => Err(IncompatibleValueTypeError::new(value, "text")),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = IncompatibleValueTypeError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(text) => Ok(&text.value),
            _ => Err(IncompatibleValueTypeError::new(value, "text")),
        }
    }
}

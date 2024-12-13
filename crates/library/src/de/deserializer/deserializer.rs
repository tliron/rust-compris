use super::{
    super::{super::normal::*, errors::*},
    enum_deserializer::*,
    map_deserializer::*,
    seq_deserializer::*,
};

use serde::de;

//
// Deserializer
//

/// Serde deserializer for Compris normal value types.
///
/// Will convert number types only if information is not lost. Otherwise, will return an error.
pub struct Deserializer<'a> {
    value: &'a Value,
}

impl<'a> Deserializer<'a> {
    /// Constructor
    pub fn new(value: &'a Value) -> Self {
        Self { value }
    }

    fn incompatible_type_error(&self) -> DeserializationError {
        DeserializationError::incompatible_type(&self.value)
    }

    fn incompatible_value_error(&self) -> DeserializationError {
        DeserializationError::incompatible_value(&self.value)
    }
}

// See: https://serde.rs/impl-deserializer.html

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializationError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Nothing => todo!(),
            Value::Null(_) => self.deserialize_unit(visitor),
            Value::Integer(_) => self.deserialize_i64(visitor),
            Value::UnsignedInteger(_) => self.deserialize_u64(visitor),
            Value::Float(_) => self.deserialize_f64(visitor),
            Value::Boolean(_) => self.deserialize_bool(visitor),
            Value::Text(_) => self.deserialize_str(visitor),
            Value::Bytes(_) => self.deserialize_bytes(visitor),
            Value::List(_) => self.deserialize_seq(visitor),
            Value::Map(_) => self.deserialize_map(visitor),
        }
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Boolean(boolean) => visitor.visit_bool(boolean.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Integer(integer) => match num_traits::cast::<_, i8>(integer.value) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, i8>(unsigned_integer.value) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if float.fract() == 0. {
                    match num_traits::cast::<_, i8>(float) {
                        Some(integer) => visitor.visit_i8(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Integer(integer) => match num_traits::cast::<_, i16>(integer.value) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, i16>(unsigned_integer.value) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if float.fract() == 0. {
                    match num_traits::cast::<_, i16>(float) {
                        Some(integer) => visitor.visit_i16(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Integer(integer) => match num_traits::cast::<_, i32>(integer.value) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, i32>(unsigned_integer.value) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if float.fract() == 0. {
                    match num_traits::cast::<_, i32>(float) {
                        Some(integer) => visitor.visit_i32(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Integer(integer) => visitor.visit_i64(integer.value),

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, i64>(unsigned_integer.value) {
                Some(integer) => visitor.visit_i64(integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if float.fract() == 0. {
                    match num_traits::cast::<_, i64>(float) {
                        Some(integer) => visitor.visit_i64(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, u8>(unsigned_integer.value) {
                Some(unsigned_integer) => visitor.visit_u8(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Integer(integer) => {
                if integer.value >= 0 {
                    match num_traits::cast::<_, u8>(integer.value) {
                        Some(insigned_integer) => visitor.visit_u8(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match num_traits::cast::<_, u8>(float) {
                        Some(unsigned_integer) => visitor.visit_u8(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, u16>(unsigned_integer.value) {
                Some(unsigned_integer) => visitor.visit_u16(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Integer(integer) => {
                if integer.value >= 0 {
                    match num_traits::cast::<_, u16>(integer.value) {
                        Some(insigned_integer) => visitor.visit_u16(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match num_traits::cast::<_, u16>(float) {
                        Some(unsigned_integer) => visitor.visit_u16(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, u32>(unsigned_integer.value) {
                Some(unsigned_integer) => visitor.visit_u32(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Value::Integer(integer) => {
                if integer.value >= 0 {
                    match num_traits::cast::<_, u32>(integer.value) {
                        Some(insigned_integer) => visitor.visit_u32(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match num_traits::cast::<_, u32>(float) {
                        Some(unsigned_integer) => visitor.visit_u32(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::UnsignedInteger(unsigned_integer) => visitor.visit_u64(unsigned_integer.value),

            Value::Integer(integer) => {
                if integer.value >= 0 {
                    match num_traits::cast::<_, u64>(integer.value) {
                        Some(insigned_integer) => visitor.visit_u64(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Value::Float(float) => {
                let float: f64 = float.value.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match num_traits::cast::<_, u64>(float) {
                        Some(unsigned_integer) => visitor.visit_u64(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Float(float) => {
                let float: f64 = float.value.into();
                match num_traits::cast::<_, f32>(float) {
                    Some(float) => visitor.visit_f32(float),
                    None => Err(self.incompatible_value_error()),
                }
            }

            Value::Integer(integer) => match num_traits::cast::<_, f32>(integer.value) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, f32>(unsigned_integer.value) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Float(float) => visitor.visit_f64(float.value.into()),

            Value::Integer(integer) => match num_traits::cast::<_, f64>(integer.value) {
                Some(float) => visitor.visit_f64(float),
                None => Err(self.incompatible_value_error()),
            },

            Value::UnsignedInteger(unsigned_integer) => match num_traits::cast::<_, f64>(unsigned_integer.value) {
                Some(float) => visitor.visit_f64(float),
                None => Err(self.incompatible_value_error()),
            },

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(DeserializationError::NotSupported("deserialize_char"))
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Text(string) => visitor.visit_str(&string.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Text(string) => visitor.visit_string(string.value.clone()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Bytes(bytes) => visitor.visit_bytes(&bytes.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Bytes(bytes) => visitor.visit_byte_buf(bytes.value.clone()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Null(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Null(_) => visitor.visit_unit(),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::List(list) => Ok(visitor.visit_seq(SeqDeserializer::new(list))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Map(map) => Ok(visitor.visit_map(MapDeserializer::new(map))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self.value {
            Value::Map(map) => Ok(visitor.visit_enum(EnumDeserializer::new(map)?)?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
}

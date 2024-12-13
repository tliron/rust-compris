use super::{
    super::{super::normal::*, errors::*},
    enum_deserializer::*,
    map_as_list_deserializer::*,
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
///
/// See [NumCast::from](num_traits::cast::NumCast::from).
pub struct Deserializer<'own> {
    value: &'own Value,
}

impl<'own> Deserializer<'own> {
    /// Constructor
    pub fn new(value: &'own Value) -> Self {
        Self { value }
    }

    fn incompatible_type_error(&self) -> DeserializeError {
        DeserializeError::incompatible_type(&self.value)
    }

    fn incompatible_value_error(&self) -> DeserializeError {
        DeserializeError::incompatible_value(&self.value)
    }
}

// See: https://serde.rs/impl-deserializer.html

impl<'de, 'own> de::Deserializer<'de> for &'own mut Deserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Nothing => Err(self.incompatible_type_error()),
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

    fn deserialize_bool<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Boolean(boolean) => visitor.visit_bool(boolean.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i8<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_i16<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_i32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_i64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_u8<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_u16<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_u32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_u64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_f32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_f64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
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

    fn deserialize_char<VisitorT>(self, _visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Err(DeserializeError::NotSupported("deserialize_char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Value::Text(string) => visitor.visit_str(&string.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_string<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Text(string) => visitor.visit_string(string.value.clone()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_bytes<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Bytes(bytes) => visitor.visit_bytes(&bytes.value),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_byte_buf<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Bytes(bytes) => visitor.visit_byte_buf(bytes.value.clone()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_option<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Null(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Null(_) => visitor.visit_unit(),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_unit_struct<VisitorT>(
        self,
        _name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<VisitorT>(
        self,
        _name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::List(list) => Ok(visitor.visit_seq(SeqDeserializer::new(list))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_tuple<VisitorT>(self, _len: usize, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<VisitorT>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Map(map) => Ok(visitor.visit_map(MapDeserializer::new(map))?),
            Value::List(list) => Ok(visitor.visit_map(MapAsListDeserializer::new(list))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_struct<VisitorT>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<VisitorT>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.value {
            Value::Map(map) => Ok(visitor.visit_enum(EnumDeserializer::new(map)?)?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_identifier<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

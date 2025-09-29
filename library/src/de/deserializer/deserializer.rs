use super::{
    super::{super::normal::*, errors::*},
    enum_deserializer::*,
    map_as_list_deserializer::*,
    map_deserializer::*,
    seq_deserializer::*,
};

use {num_traits::*, serde::de};

//
// Deserializer
//

/// Serde deserializer for Compris normal types.
///
/// Will convert number types only if information is not lost. Otherwise, will return an error.
///
/// See [NumCast::from](cast::NumCast::from).
pub struct Deserializer<'own, AnnotatedT> {
    variant: &'own Variant<AnnotatedT>,
}

impl<'own, AnnotatedT> Deserializer<'own, AnnotatedT> {
    /// Constructor
    pub fn new(variant: &'own Variant<AnnotatedT>) -> Self {
        Self { variant }
    }

    fn incompatible_type_error(&self) -> DeserializeError {
        DeserializeError::incompatible_type(&self.variant)
    }

    fn incompatible_value_error(&self) -> DeserializeError {
        DeserializeError::incompatible_variant(&self.variant)
    }
}

// See: https://serde.rs/impl-deserializer.html

impl<'de, 'own, AnnotatedT> de::Deserializer<'de> for &'own mut Deserializer<'de, AnnotatedT> {
    type Error = DeserializeError;

    fn deserialize_any<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Undefined => Err(self.incompatible_type_error()),
            Variant::Null(_) => self.deserialize_unit(visitor),
            Variant::Integer(_) => self.deserialize_i64(visitor),
            Variant::UnsignedInteger(_) => self.deserialize_u64(visitor),
            Variant::Float(_) => self.deserialize_f64(visitor),
            Variant::Boolean(_) => self.deserialize_bool(visitor),
            Variant::Text(_) => self.deserialize_str(visitor),
            Variant::Blob(_) => self.deserialize_bytes(visitor),
            Variant::List(_) => self.deserialize_seq(visitor),
            Variant::Map(_) => self.deserialize_map(visitor),
        }
    }

    fn deserialize_bool<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Boolean(boolean) => visitor.visit_bool(boolean.inner),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i8<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Integer(integer) => match cast(integer.inner) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Float(float) => {
                let float: f64 = float.inner.into();
                if float.fract() == 0. {
                    match cast(float) {
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
        match self.variant {
            Variant::Integer(integer) => match cast(integer.inner) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Float(float) => {
                let float: f64 = float.into();
                if float.fract() == 0. {
                    match cast(float) {
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
        match self.variant {
            Variant::Integer(integer) => match cast(integer.inner) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Float(float) => {
                let float: f64 = float.into();
                if float.fract() == 0. {
                    match cast(float) {
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
        match self.variant {
            Variant::Integer(integer) => visitor.visit_i64(integer.inner),

            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(integer) => visitor.visit_i64(integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Float(float) => {
                let float: f64 = float.inner.into();
                if float.fract() == 0. {
                    match cast(float) {
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
        match self.variant {
            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(unsigned_integer) => visitor.visit_u8(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Integer(integer) => {
                if integer.inner >= 0 {
                    match cast(integer.inner) {
                        Some(insigned_integer) => visitor.visit_u8(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Variant::Float(float) => {
                let float: f64 = float.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match cast(float) {
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
        match self.variant {
            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(unsigned_integer) => visitor.visit_u16(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Integer(integer) => {
                if integer.inner >= 0 {
                    match cast(integer.inner) {
                        Some(insigned_integer) => visitor.visit_u16(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Variant::Float(float) => {
                let float: f64 = float.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match cast(float) {
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
        match self.variant {
            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
                Some(unsigned_integer) => visitor.visit_u32(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Variant::Integer(integer) => {
                if integer.inner >= 0 {
                    match cast(integer.inner) {
                        Some(insigned_integer) => visitor.visit_u32(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Variant::Float(float) => {
                let float: f64 = float.inner.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match cast(float) {
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
        match self.variant {
            Variant::UnsignedInteger(unsigned_integer) => visitor.visit_u64(unsigned_integer.inner),

            Variant::Integer(integer) => {
                if integer.inner >= 0 {
                    match cast(integer.inner) {
                        Some(insigned_integer) => visitor.visit_u64(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Variant::Float(float) => {
                let float: f64 = float.into();
                if (float >= 0.) && (float.fract() == 0.) {
                    match cast(float) {
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
        match self.variant {
            Variant::Float(float) => {
                let float: f64 = float.into();
                match cast(float) {
                    Some(float) => visitor.visit_f32(float),
                    None => Err(self.incompatible_value_error()),
                }
            }

            Variant::Integer(integer) => match cast(integer.inner) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            Variant::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer.inner) {
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
        match self.variant {
            Variant::Float(float) => visitor.visit_f64(float.into()),

            Variant::Integer(integer) => match cast(integer.inner) {
                Some(float) => visitor.visit_f64(float),
                None => Err(self.incompatible_value_error()),
            },

            Variant::UnsignedInteger(unsigned_integer) => match cast::<_, f64>(unsigned_integer.inner) {
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
        match self.variant {
            Variant::Text(text) => visitor.visit_str(text.into()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_string<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Text(text) => visitor.visit_str(text.into()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_bytes<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Blob(blob) => visitor.visit_bytes(blob.into()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_byte_buf<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Blob(blob) => visitor.visit_bytes(blob.into()),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_option<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Null(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.variant {
            Variant::Null(_) => visitor.visit_unit(),
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
        match self.variant {
            Variant::List(list) => Ok(visitor.visit_seq(SeqDeserializer::new(list))?),
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
        match self.variant {
            Variant::Map(map) => Ok(visitor.visit_map(MapDeserializer::new(map))?),
            Variant::List(list) => Ok(visitor.visit_map(MapAsListDeserializer::new(list))?),
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
        match self.variant {
            Variant::Map(map) => Ok(visitor.visit_enum(EnumDeserializer::new(map)?)?),
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

use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
};

use serde::{de, Deserializer as _};

//
// VariantDeserializer
//

pub(crate) struct VariantDeserializer<'de> {
    value: &'de Value,
}

impl<'de> VariantDeserializer<'de> {
    pub fn new(value: &'de Value) -> Self {
        Self { value }
    }
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = DeserializationError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(DeserializationError::incompatible_value(self.value))
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
        seed.deserialize(&mut Deserializer::new(self.value))
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        Deserializer::new(self.value).deserialize_tuple(len, visitor)
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Deserializer::new(self.value).deserialize_struct("", fields, visitor)
    }
}

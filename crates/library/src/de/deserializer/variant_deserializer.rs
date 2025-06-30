use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
};

use serde::{Deserializer as _, de};

//
// VariantDeserializer
//

pub(crate) struct VariantDeserializer<'de, AnnotatedT> {
    value: &'de Value<AnnotatedT>,
}

impl<'de, AnnotatedT> VariantDeserializer<'de, AnnotatedT> {
    pub fn new(value: &'de Value<AnnotatedT>) -> Self {
        Self { value }
    }
}

impl<'de, AnnotatedT> de::VariantAccess<'de> for VariantDeserializer<'de, AnnotatedT> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(DeserializeError::incompatible_value(self.value))
    }

    fn newtype_variant_seed<SeedT>(self, seed: SeedT) -> Result<SeedT::Value, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::new(self.value))
    }

    fn tuple_variant<VisitorT>(self, len: usize, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Deserializer::new(self.value).deserialize_tuple(len, visitor)
    }

    fn struct_variant<VisitorT>(
        self,
        fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Deserializer::new(self.value).deserialize_struct("", fields, visitor)
    }
}

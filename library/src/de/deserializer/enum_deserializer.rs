use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
    variant_deserializer::*,
};

use serde::de;

//
// EnumDeserializer
//

pub(crate) struct EnumDeserializer<'de, AnnotatedT> {
    key: &'de Variant<AnnotatedT>,
    value: &'de Variant<AnnotatedT>,
}

impl<'de, AnnotatedT> EnumDeserializer<'de, AnnotatedT> {
    pub(crate) fn new(map: &'de Map<AnnotatedT>) -> Result<Self, DeserializeError> {
        if map.inner.len() == 1 {
            let (key, value) = map.inner.iter().next().expect("non-empty");
            Ok(Self { key, value })
        } else {
            Err(DeserializeError::IncompatibleVariant(format!("map length is not 1: {}", map)))
        }
    }
}

impl<'de, AnnotatedT> de::EnumAccess<'de> for EnumDeserializer<'de, AnnotatedT> {
    type Error = DeserializeError;
    type Variant = VariantDeserializer<'de, AnnotatedT>;

    fn variant_seed<SeedT>(self, seed: SeedT) -> Result<(SeedT::Value, Self::Variant), Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut Deserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}

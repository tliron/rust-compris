use super::{super::*, deserializer::*, errors::*, variant_deserializer::*};

//
// EnumDeserializer
//

pub(crate) struct EnumDeserializer<'de> {
    key: &'de Value,
    value: &'de Value,
}

impl<'de> EnumDeserializer<'de> {
    pub(crate) fn new(map: &'de Map) -> Result<Self, DeserializationError> {
        if map.value.len() == 1 {
            let (key, value) = map.value.iter().next().unwrap();
            Ok(Self { key, value })
        } else {
            Err(DeserializationError::IncompatibleValue(format!("map length is not 1: {}", map.to_debug_string()?)))
        }
    }
}

impl<'de> serde::de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = DeserializationError;

    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V: serde::de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let key = seed.deserialize(&mut Deserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}

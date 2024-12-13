use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
    variant_deserializer::*,
};

use serde::de;

//
// EnumDeserializer
//

pub(crate) struct EnumDeserializer<'de> {
    key: &'de Value,
    value: &'de Value,
}

impl<'de> EnumDeserializer<'de> {
    pub(crate) fn new(map: &'de Map) -> Result<Self, DeserializeError> {
        if map.value.len() == 1 {
            let (key, value) = map.value.iter().next().unwrap();
            Ok(Self { key, value })
        } else {
            Err(DeserializeError::IncompatibleValue(format!("map length is not 1: {}", map)))
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = DeserializeError;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<SeedT>(self, seed: SeedT) -> Result<(SeedT::Value, Self::Variant), Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut Deserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}

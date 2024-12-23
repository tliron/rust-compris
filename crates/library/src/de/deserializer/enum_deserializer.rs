use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
    variant_deserializer::*,
};

use {kutil_cli::debug::*, serde::de};

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

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = DeserializationError;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<SeedT>(self, seed: SeedT) -> Result<(SeedT::Value, Self::Variant), Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut Deserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}

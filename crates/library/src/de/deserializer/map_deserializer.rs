use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
};

use {serde::de, std::collections::*};

//
// MapDeserializer
//

pub(crate) struct MapDeserializer<'de> {
    iterator: btree_map::Iter<'de, Value, Value>,
    current_entry: Option<(&'de Value, &'de Value)>,
}

impl<'de> MapDeserializer<'de> {
    pub(crate) fn new(map: &'de Map) -> Self {
        Self { iterator: map.value.iter(), current_entry: None }
    }

    fn next(&mut self) {
        self.current_entry = self.iterator.next();
    }
}

impl<'de> de::MapAccess<'de> for MapDeserializer<'de> {
    type Error = DeserializeError;

    fn next_key_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next();
        match self.current_entry {
            Some((key, _)) => Ok(Some(seed.deserialize(&mut Deserializer::new(key))?)),
            None => Ok(None),
        }
    }

    fn next_value_seed<SeedT>(&mut self, seed: SeedT) -> Result<SeedT::Value, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        match self.current_entry {
            Some((_, value)) => Ok(seed.deserialize(&mut Deserializer::new(value))?),
            None => Err(DeserializeError::NoMoreItems), // this shouldn't happen, but still
        }
    }
}

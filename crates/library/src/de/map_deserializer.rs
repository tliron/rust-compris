use super::{super::*, deserializer::*, errors::*};

use ordermap::map::Iter;

//
// MapDeserializer
//

pub(crate) struct MapDeserializer<'de> {
    iterator: Iter<'de, Value, Value>,
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

impl<'de> serde::de::MapAccess<'de> for MapDeserializer<'de> {
    type Error = DeserializationError;

    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        self.next();
        match self.current_entry {
            Some((key, _)) => Ok(Some(seed.deserialize(&mut Deserializer::new(key))?)),
            None => Ok(None),
        }
    }

    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        match self.current_entry {
            Some((_, value)) => Ok(seed.deserialize(&mut Deserializer::new(value))?),
            None => Err(DeserializationError::NoMoreElements), // this shouldn't happen, but still
        }
    }
}

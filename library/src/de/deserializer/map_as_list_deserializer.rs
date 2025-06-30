use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
};

use {serde::de, std::slice::*};

//
// MapAsListDeserializer
//

pub(crate) struct MapAsListDeserializer<'de, AnnotatedT> {
    iterator: Iter<'de, Variant<AnnotatedT>>,
    current_entry: Option<(&'de Variant<AnnotatedT>, &'de Variant<AnnotatedT>)>,
}

impl<'de, AnnotatedT> MapAsListDeserializer<'de, AnnotatedT> {
    pub(crate) fn new(list: &'de List<AnnotatedT>) -> Self {
        Self { iterator: list.inner.iter(), current_entry: None }
    }

    fn next(&mut self) -> Result<(), DeserializeError> {
        let current_entry = self.iterator.next();

        match current_entry {
            Some(current_entry) => {
                self.current_entry = current_entry.to_pair();
                if self.current_entry.is_none() {
                    return Err(DeserializeError::incompatible_variant(current_entry));
                }
            }

            None => {
                self.current_entry = None;
            }
        }

        Ok(())
    }
}

impl<'de, AnnotatedT> de::MapAccess<'de> for MapAsListDeserializer<'de, AnnotatedT> {
    type Error = DeserializeError;

    fn next_key_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next()?;
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

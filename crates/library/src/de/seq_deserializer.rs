use super::{super::*, deserializer::*, errors::*};

use {serde::de, std::slice::Iter};

//
// SeqDeserializer
//

pub(crate) struct SeqDeserializer<'de> {
    iterator: Iter<'de, Value>,
    current_element: Option<&'de Value>,
}

impl<'de> SeqDeserializer<'de> {
    pub(crate) fn new(list: &'de List) -> Self {
        Self { iterator: list.value.iter(), current_element: None }
    }

    fn next(&mut self) {
        self.current_element = self.iterator.next();
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = DeserializationError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
        self.next();
        match self.current_element {
            Some(element) => Ok(Some(seed.deserialize(&mut Deserializer::new(element))?)),
            None => Ok(None),
        }
    }
}

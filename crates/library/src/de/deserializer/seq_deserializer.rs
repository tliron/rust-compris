use super::{
    super::{super::normal::*, errors::*},
    deserializer::*,
};

use {serde::de, std::slice::*};

//
// SeqDeserializer
//

pub(crate) struct SeqDeserializer<'de, AnnotationsT> {
    iterator: Iter<'de, Value<AnnotationsT>>,
    current_item: Option<&'de Value<AnnotationsT>>,
}

impl<'de, AnnotationsT> SeqDeserializer<'de, AnnotationsT> {
    pub(crate) fn new(list: &'de List<AnnotationsT>) -> Self {
        Self { iterator: list.value.iter(), current_item: None }
    }

    fn next(&mut self) {
        self.current_item = self.iterator.next();
    }
}

impl<'de, AnnotationsT> de::SeqAccess<'de> for SeqDeserializer<'de, AnnotationsT> {
    type Error = DeserializeError;

    fn next_element_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next();
        match self.current_item {
            Some(item) => Ok(Some(seed.deserialize(&mut Deserializer::new(item))?)),
            None => Ok(None),
        }
    }
}

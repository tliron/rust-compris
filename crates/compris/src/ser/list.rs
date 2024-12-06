use super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// List
//

impl List {
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> ListWithSerializationMode<'a> {
        ListWithSerializationMode::new(self, serialization_mode)
    }

    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
        for element in &self.value {
            seq.serialize_element(&element.with_serialization_mode(serialization_mode))?;
        }
        seq.end()
    }
}

impl Serialize for List {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
        for element in &self.value {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

//
// ListWithSerializationMode
//

pub struct ListWithSerializationMode<'a> {
    pub list: &'a List,
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> ListWithSerializationMode<'a> {
    pub fn new(list: &'a List, serialization_mode: &'a SerializationMode) -> Self {
        Self { list, serialization_mode }
    }
}

impl<'a> Serialize for ListWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.list.serialize_with_mode(serializer, self.serialization_mode)
    }
}

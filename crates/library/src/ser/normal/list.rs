use super::super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// List
//

impl List {
    /// Adds [SerializationMode] support.
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> ListWithSerializationMode<'a> {
        ListWithSerializationMode::new(self, serialization_mode)
    }

    /// Serializes according to the [SerializationMode].
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

/// Adds [SerializationMode] support to [List]. The mode will be applied recursively
/// to list elements.
pub struct ListWithSerializationMode<'a> {
    /// Wrapped value.
    pub list: &'a List,

    /// Serialization mode.
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> ListWithSerializationMode<'a> {
    /// Constructor.
    pub fn new(list: &'a List, serialization_mode: &'a SerializationMode) -> Self {
        Self { list, serialization_mode }
    }
}

impl<'a> Serialize for ListWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.list.serialize_with_mode(serializer, self.serialization_mode)
    }
}

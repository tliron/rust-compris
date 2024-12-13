use super::super::{super::normal::*, modal::*, mode::*, serializer::Serializer as ComprisSerializer};

use serde::ser::*;

impl Serialize for List {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
        for item in &self.value {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl SerializeModalRescursive for List {
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        compris_serializer: &ComprisSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
        for item in &self.value {
            seq.serialize_element(&item.modal(mode, compris_serializer))?;
        }
        seq.end()
    }
}

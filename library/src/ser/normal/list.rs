use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
    serializer::Serializer as ComprisSerializer,
};

use serde::ser::*;

impl<AnnotatedT> Serialize for List<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
        for item in &self.inner {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<AnnotatedT> SerializeModalRescursive for List<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        compris_serializer: &ComprisSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.inner.len()))?;
        for item in &self.inner {
            seq.serialize_element(&item.modal(mode, compris_serializer))?;
        }
        seq.end()
    }
}

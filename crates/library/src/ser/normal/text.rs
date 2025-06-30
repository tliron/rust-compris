use super::super::super::normal::*;

use serde::ser::*;

impl<AnnotatedT> Serialize for Text<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

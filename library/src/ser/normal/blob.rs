use super::super::{super::normal::*, modal::*, mode::*};

use serde::ser::*;

impl<AnnotatedT> Serialize for Blob<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_bytes(&*self.inner)
    }
}

impl<AnnotatedT> SerializeModal for Blob<AnnotatedT> {
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match &mode.blob {
            BlobSerializationMode::AsBytes => serializer.serialize_bytes(&*self.inner),

            BlobSerializationMode::StringifyBase64(hint) => {
                let string = self.to_base64();
                match hint {
                    None => serializer.serialize_str(&string),

                    Some(hint) => {
                        let mut map = serializer.serialize_map(Some(1))?;
                        map.serialize_entry(&hint, &string)?;
                        map.end()
                    }
                }
            }
        }
    }
}

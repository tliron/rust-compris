use super::super::{super::normal::*, mode::*, modal::*};

use serde::ser::*;

impl Serialize for Bytes {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_bytes(&*self.value)
    }
}

impl SerializeModal for Bytes {
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match &mode.bytes {
            BytesSerializationMode::AsBytes => serializer.serialize_bytes(&*self.value),

            BytesSerializationMode::StringifyBase64(hint) => {
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

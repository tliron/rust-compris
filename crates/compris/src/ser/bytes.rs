use super::{super::*, serialization_mode::*};

use {base64::prelude::*, serde::ser::*};

//
// Bytes
//

impl Bytes {
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> BytesWithSerializationMode<'a> {
        BytesWithSerializationMode::new(self, serialization_mode)
    }

    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        match &serialization_mode.bytes {
            BytesSerializationMode::AsBytes => serializer.serialize_bytes(&*self.value),

            BytesSerializationMode::AsBase64(hint) => {
                let string = BASE64_STANDARD.encode(&self.value);
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

impl Serialize for Bytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&*self.value)
    }
}

//
// BytesWithSerializationMode
//

pub struct BytesWithSerializationMode<'a> {
    pub bytes: &'a Bytes,
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> BytesWithSerializationMode<'a> {
    pub fn new(bytes: &'a Bytes, serialization_mode: &'a SerializationMode) -> Self {
        Self { bytes, serialization_mode }
    }
}

impl<'a> Serialize for BytesWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bytes.serialize_with_mode(serializer, self.serialization_mode)
    }
}

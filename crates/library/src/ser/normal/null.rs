use super::super::super::normal::*;

use serde::ser::*;

impl Serialize for Null {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_unit()
    }
}

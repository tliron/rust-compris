use super::super::{
    super::{annotation::*, normal::*},
    modal::*,
    mode::*,
    serializer::Serializer as ComprisSerializer,
};

use serde::ser::*;

//
// Value
//

impl<AnnotatedT> Serialize for Value<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match self {
            Self::Nothing => Err(Error::custom("value is nothing")),
            Self::Null(null) => null.serialize(serializer),
            Self::Integer(integer) => integer.serialize(serializer),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.serialize(serializer),
            Self::Float(float) => float.serialize(serializer),
            Self::Boolean(boolean) => boolean.serialize(serializer),
            Self::Text(text) => text.serialize(serializer),
            Self::Blob(blob) => blob.serialize(serializer),
            Self::List(list) => list.serialize(serializer),
            Self::Map(map) => map.serialize(serializer),
        }
    }
}

impl<AnnotatedT> SerializeModalRescursive for Value<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        modal_serializer: &ComprisSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match &self {
            Value::Nothing => Err(Error::custom("value is nothing")),
            Value::Null(null) => null.serialize(serializer),
            Value::Integer(integer) => integer.modal(mode).serialize(serializer),
            Value::UnsignedInteger(unsigned_integer) => unsigned_integer.modal(mode).serialize(serializer),
            Value::Float(float) => float.modal(mode).serialize(serializer),
            Value::Boolean(boolean) => boolean.serialize(serializer),
            Value::Text(text) => text.serialize(serializer),
            Value::Blob(blob) => blob.modal(mode).serialize(serializer),
            Value::List(list) => list.modal(mode, modal_serializer).serialize(serializer),
            Value::Map(map) => map.modal(mode, modal_serializer).serialize(serializer),
        }
    }
}

use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
    serializer::Serializer as ComprisSerializer,
};

use serde::ser::*;

impl<AnnotatedT> Serialize for Variant<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match self {
            Self::Undefined => Err(Error::custom("variant is undefined")),
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

impl<AnnotatedT> SerializeModalRescursive for Variant<AnnotatedT>
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
            Variant::Undefined => Err(Error::custom("variant is undefined")),
            Variant::Null(null) => null.serialize(serializer),
            Variant::Integer(integer) => integer.modal(mode).serialize(serializer),
            Variant::UnsignedInteger(unsigned_integer) => unsigned_integer.modal(mode).serialize(serializer),
            Variant::Float(float) => float.modal(mode).serialize(serializer),
            Variant::Boolean(boolean) => boolean.serialize(serializer),
            Variant::Text(text) => text.serialize(serializer),
            Variant::Blob(blob) => blob.modal(mode).serialize(serializer),
            Variant::List(list) => list.modal(mode, modal_serializer).serialize(serializer),
            Variant::Map(map) => map.modal(mode, modal_serializer).serialize(serializer),
        }
    }
}

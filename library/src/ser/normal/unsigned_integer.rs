use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
};

use serde::ser::*;

impl<AnnotatedT> Serialize for UnsignedInteger<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_u64(self.inner)
    }
}

impl<AnnotatedT> SerializeModal for UnsignedInteger<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        // See: https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from
        match &mode.unsigned_integer {
            UnsignedIntegerSerializationMode::AsU64 => serializer.serialize_u64(self.inner),

            UnsignedIntegerSerializationMode::AsI64 => {
                let integer = num_traits::cast(self.inner)
                    .ok_or_else(|| Error::custom(format!("cannot cast to i64: {}", self.inner)))?;
                if (integer >= 0) && (mode.integer == IntegerSerializationMode::AsU64IfNonNegative) {
                    // Avoid endless recursion!
                    serializer.serialize_i64(integer)
                } else {
                    Integer::<AnnotatedT>::from(integer).with_annotations_from(self).serialize_modal(serializer, mode)
                }
            }

            UnsignedIntegerSerializationMode::AsF64 => {
                let float: f64 = num_traits::cast(self.inner)
                    .ok_or_else(|| Error::custom(format!("cannot cast to f64: {}", self.inner)))?;
                Float::<AnnotatedT>::from(float).with_annotations_from(self).serialize_modal(serializer, mode)
            }

            UnsignedIntegerSerializationMode::Stringify(hint) => {
                let string = self.inner.to_string();
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

use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
};

use serde::ser::*;

impl<AnnotatedT> Serialize for Integer<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_i64(self.inner)
    }
}

impl<AnnotatedT> SerializeModal for Integer<AnnotatedT>
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
        match &mode.integer {
            IntegerSerializationMode::AsI64 => serializer.serialize_i64(self.inner),

            IntegerSerializationMode::AsU64IfNonNegative => {
                if self.inner < 0 {
                    serializer.serialize_i64(self.inner)
                } else {
                    let unsigned_integer = self.inner as u64; // should always succeed
                    if mode.unsigned_integer.might_be_integer() {
                        // Avoid endless recursion!
                        serializer.serialize_u64(unsigned_integer)
                    } else {
                        UnsignedInteger::<AnnotatedT>::from(unsigned_integer)
                            .with_annotations_from(self)
                            .serialize_modal(serializer, mode)
                    }
                }
            }

            IntegerSerializationMode::AsF64 => {
                let float = num_traits::cast(self.inner)
                    .ok_or_else(|| Error::custom(format!("cannot cast to f64: {}", self.inner)))?;
                if mode.float.might_be_integer() {
                    // Avoid endless recursion!
                    serializer.serialize_f64(float)
                } else {
                    Float::<AnnotatedT>::from(float).with_annotations_from(self).serialize_modal(serializer, mode)
                }
            }

            IntegerSerializationMode::Stringify(hint) => {
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

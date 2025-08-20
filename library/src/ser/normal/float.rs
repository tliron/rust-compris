use super::super::{
    super::{annotate::*, normal::*},
    modal::*,
    mode::*,
};

use serde::ser::*;

impl<AnnotatedT> Serialize for Float<AnnotatedT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_f64(self.into())
    }
}

impl<AnnotatedT> SerializeModal for Float<AnnotatedT>
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
        match &mode.float {
            FloatSerializationMode::AsF64 => serializer.serialize_f64(self.into()),

            FloatSerializationMode::AsI64 => {
                let float: f64 = self.inner.trunc().into();
                let integer =
                    num_traits::cast(float).ok_or_else(|| Error::custom(format!("cannot cast to i64: {}", float)))?;
                if mode.integer.might_be_float() {
                    // Avoid endless recursion!
                    serializer.serialize_i64(integer)
                } else {
                    Integer::<AnnotatedT>::from(integer).with_annotations_from(self).serialize_modal(serializer, mode)
                }
            }

            FloatSerializationMode::AsI64IfWhole => {
                if self.inner.fract() == 0.0 {
                    match num_traits::cast(self.inner) {
                        Some(integer) => {
                            if mode.integer.might_be_float() {
                                // Avoid endless recursion!
                                serializer.serialize_i64(integer)
                            } else {
                                Integer::<AnnotatedT>::from(integer)
                                    .with_annotations_from(self)
                                    .serialize_modal(serializer, mode)
                            }
                        }

                        None => serializer.serialize_f64(self.into()),
                    }
                } else {
                    serializer.serialize_f64(self.into())
                }
            }

            FloatSerializationMode::Stringify(hint) => {
                let string = self.inner.to_string();
                match hint {
                    None => serializer.serialize_str(&string),

                    Some(key) => {
                        let mut map = serializer.serialize_map(Some(1))?;
                        map.serialize_entry(&key, &string)?;
                        map.end()
                    }
                }
            }
        }
    }
}

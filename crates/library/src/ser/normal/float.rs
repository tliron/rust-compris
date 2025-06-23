use super::super::{
    super::{annotation::*, normal::*},
    modal::*,
    mode::*,
};

use serde::ser::*;

impl<AnnotationsT> Serialize for Float<AnnotationsT> {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_f64(self.value.into())
    }
}

impl<AnnotationsT> SerializeModal for Float<AnnotationsT>
where
    AnnotationsT: Annotated + Clone + Default,
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
            FloatSerializationMode::AsF64 => serializer.serialize_f64(self.value.into()),

            FloatSerializationMode::AsI64 => {
                let float: f64 = self.value.trunc().into();
                let integer =
                    num_traits::cast(float).ok_or_else(|| Error::custom(format!("cannot cast to i64: {}", float)))?;
                if mode.integer.might_be_float() {
                    // Avoid endless recursion!
                    serializer.serialize_i64(integer)
                } else {
                    Integer::<AnnotationsT>::new(integer).with_annotations_from(self).serialize_modal(serializer, mode)
                }
            }

            FloatSerializationMode::AsI64IfWhole => {
                if self.value.fract() == 0.0 {
                    match num_traits::cast(self.value) {
                        Some(integer) => {
                            if mode.integer.might_be_float() {
                                // Avoid endless recursion!
                                serializer.serialize_i64(integer)
                            } else {
                                Integer::<AnnotationsT>::new(integer)
                                    .with_annotations_from(self)
                                    .serialize_modal(serializer, mode)
                            }
                        }

                        None => serializer.serialize_f64(self.value.into()),
                    }
                } else {
                    serializer.serialize_f64(self.value.into())
                }
            }

            FloatSerializationMode::Stringify(hint) => {
                let string = self.value.to_string();
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

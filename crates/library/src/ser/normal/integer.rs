use super::super::{
    super::{meta::*, normal::*},
    mode::*,
    modal::*,
};

use serde::ser::*;

impl Serialize for Integer {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_i64(self.value)
    }
}

impl SerializeModal for Integer {
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
            IntegerSerializationMode::AsI64 => serializer.serialize_i64(self.value),

            IntegerSerializationMode::AsU64IfNonNegative => {
                if self.value < 0 {
                    serializer.serialize_i64(self.value)
                } else {
                    let unsigned_integer = self.value as u64; // should always succeed
                    if mode.unsigned_integer.might_be_integer() {
                        // Avoid endless recursion!
                        serializer.serialize_u64(unsigned_integer)
                    } else {
                        UnsignedInteger::new(unsigned_integer)
                            .with_meta(self.meta.clone())
                            .serialize_modal(serializer, mode)
                    }
                }
            }

            IntegerSerializationMode::AsF64 => match num_traits::cast::<_, f64>(self.value) {
                Some(float) => {
                    if mode.float.might_be_integer() {
                        // Avoid endless recursion!
                        serializer.serialize_f64(float)
                    } else {
                        Float::new(float).with_meta(self.meta.clone()).serialize_modal(serializer, mode)
                    }
                }

                None => Err(Error::custom(format!("cannot cast to f64: {}", self.value))),
            },

            IntegerSerializationMode::Stringify(hint) => {
                let string = self.value.to_string();
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

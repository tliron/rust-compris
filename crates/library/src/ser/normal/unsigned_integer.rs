use super::super::{
    super::{meta::*, normal::*},
    modal::*,
    mode::*,
};

use serde::ser::*;

impl Serialize for UnsignedInteger {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        serializer.serialize_u64(self.value)
    }
}

impl SerializeModal for UnsignedInteger {
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
            UnsignedIntegerSerializationMode::AsU64 => serializer.serialize_u64(self.value),

            UnsignedIntegerSerializationMode::AsI64 => match num_traits::cast::<_, i64>(self.value) {
                Some(integer) => {
                    if (integer >= 0) && (mode.integer == IntegerSerializationMode::AsU64IfNonNegative) {
                        // Avoid endless recursion!
                        serializer.serialize_i64(integer)
                    } else {
                        Integer::new(integer).with_meta(self.meta.clone()).serialize_modal(serializer, mode)
                    }
                }

                None => Err(Error::custom(format!("cannot cast to i64: {}", self.value))),
            },

            UnsignedIntegerSerializationMode::AsF64 => match num_traits::cast::<_, f64>(self.value) {
                Some(float) => Float::new(float).with_meta(self.meta.clone()).serialize_modal(serializer, mode),

                None => Err(Error::custom(format!("cannot cast to f64: {}", self.value))),
            },

            UnsignedIntegerSerializationMode::Stringify(hint) => {
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

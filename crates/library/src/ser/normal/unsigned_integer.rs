use super::super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// UnsignedInteger
//

impl UnsignedInteger {
    /// Adds [SerializationMode] support.
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> UnsignedIntegerWithSerializationMode<'a> {
        UnsignedIntegerWithSerializationMode::new(self, serialization_mode)
    }

    /// Serializes according to the [SerializationMode].
    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        // See: https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from
        match &serialization_mode.unsigned_integer {
            UnsignedIntegerSerializationMode::AsUnsignedInteger => serializer.serialize_u64(self.value),

            UnsignedIntegerSerializationMode::AsInteger => match num_traits::cast::<_, i64>(self.value) {
                Some(integer) => {
                    if (integer >= 0)
                        && (serialization_mode.integer == IntegerSerializationMode::AsUnsignedIntegerIfNonNegative)
                    {
                        // Avoid endless recursion!
                        serializer.serialize_i64(integer)
                    } else {
                        Integer::new(integer).with_meta(&self.meta).serialize_with_mode(serializer, serialization_mode)
                    }
                }

                None => Err(Error::custom(format!("cannot cast to i64: {}", self.value))),
            },

            UnsignedIntegerSerializationMode::AsFloat => match num_traits::cast::<_, f64>(self.value) {
                Some(float) => {
                    Float::new(float).with_meta(&self.meta).serialize_with_mode(serializer, serialization_mode)
                }

                None => Err(Error::custom(format!("cannot cast to f64: {}", self.value))),
            },

            UnsignedIntegerSerializationMode::AsString(hint) => {
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

impl Serialize for UnsignedInteger {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.value)
    }
}

//
// UnsignedIntegerWithSerializationMode
//

/// Adds [SerializationMode] support to [UnsignedInteger].
pub struct UnsignedIntegerWithSerializationMode<'a> {
    /// Wrapped value.
    pub unsigned_integer: &'a UnsignedInteger,

    /// Serialization mode.
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> UnsignedIntegerWithSerializationMode<'a> {
    /// Constructor.
    pub fn new(unsigned_integer: &'a UnsignedInteger, serialization_mode: &'a SerializationMode) -> Self {
        Self { unsigned_integer, serialization_mode }
    }
}

impl<'a> Serialize for UnsignedIntegerWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.unsigned_integer.serialize_with_mode(serializer, self.serialization_mode)
    }
}

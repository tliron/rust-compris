use super::super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// Integer
//

impl Integer {
    /// Adds [SerializationMode] support.
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> IntegerWithSerializationMode<'a> {
        IntegerWithSerializationMode::new(self, serialization_mode)
    }

    /// Serializes according to the [SerializationMode].
    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        // See: https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from
        match &serialization_mode.integer {
            IntegerSerializationMode::AsInteger => serializer.serialize_i64(self.value),

            IntegerSerializationMode::AsUnsignedIntegerIfNonNegative => {
                if self.value < 0 {
                    serializer.serialize_i64(self.value)
                } else {
                    let unsigned_integer = self.value as u64; // should always succeed
                    if serialization_mode.unsigned_integer.might_be_integer() {
                        // Avoid endless recursion!
                        serializer.serialize_u64(unsigned_integer)
                    } else {
                        UnsignedInteger::new(unsigned_integer)
                            .with_meta(&self.meta)
                            .serialize_with_mode(serializer, serialization_mode)
                    }
                }
            }

            IntegerSerializationMode::AsFloat => match num_traits::cast::<_, f64>(self.value) {
                Some(float) => {
                    if serialization_mode.float.might_be_integer() {
                        // Avoid endless recursion!
                        serializer.serialize_f64(float)
                    } else {
                        Float::new(float).with_meta(&self.meta).serialize_with_mode(serializer, serialization_mode)
                    }
                }

                None => Err(Error::custom(format!("cannot cast to f64: {}", self.value))),
            },

            IntegerSerializationMode::AsString(hint) => {
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

impl Serialize for Integer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.value)
    }
}

//
// IntegerWithSerializationMode
//

/// Adds [SerializationMode] support to [Integer].
pub struct IntegerWithSerializationMode<'a> {
    /// Wrapped value.
    pub integer: &'a Integer,

    /// Serialization mode.
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> IntegerWithSerializationMode<'a> {
    /// Constructor.
    pub fn new(integer: &'a Integer, serialization_mode: &'a SerializationMode) -> Self {
        Self { integer, serialization_mode }
    }
}

impl<'a> Serialize for IntegerWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.integer.serialize_with_mode(serializer, self.serialization_mode)
    }
}

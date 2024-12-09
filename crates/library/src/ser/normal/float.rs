use super::super::{super::*, serialization_mode::*};

use serde::ser::*;

//
// Float
//

impl Float {
    /// Adds [SerializationMode] support.
    pub fn with_serialization_mode<'a>(
        &'a self,
        serialization_mode: &'a SerializationMode,
    ) -> FloatWithSerializationMode<'a> {
        FloatWithSerializationMode::new(self, serialization_mode)
    }

    /// Serializes according to the [SerializationMode].
    pub fn serialize_with_mode<S: Serializer>(
        &self,
        serializer: S,
        serialization_mode: &SerializationMode,
    ) -> Result<S::Ok, S::Error> {
        // See: https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from
        match &serialization_mode.float {
            FloatSerializationMode::AsFloat => serializer.serialize_f64(self.value.into()),

            FloatSerializationMode::AsInteger => {
                let float: f64 = self.value.trunc().into();
                match num_traits::cast::<_, i64>(float) {
                    Some(integer) => {
                        if serialization_mode.integer.might_be_float() {
                            // Avoid endless recursion!
                            serializer.serialize_i64(integer)
                        } else {
                            Integer::new(integer)
                                .with_meta(&self.meta)
                                .serialize_with_mode(serializer, serialization_mode)
                        }
                    }

                    None => Err(Error::custom(format!("cannot cast to i64: {}", float))),
                }
            }

            FloatSerializationMode::AsIntegerIfFractionless => {
                if self.value.fract() == 0.0 {
                    match num_traits::cast::<_, i64>(self.value) {
                        Some(integer) => {
                            if serialization_mode.integer.might_be_float() {
                                // Avoid endless recursion!
                                serializer.serialize_i64(integer)
                            } else {
                                Integer::new(integer)
                                    .with_meta(&self.meta)
                                    .serialize_with_mode(serializer, serialization_mode)
                            }
                        }

                        None => serializer.serialize_f64(self.value.into()),
                    }
                } else {
                    serializer.serialize_f64(self.value.into())
                }
            }

            FloatSerializationMode::AsString(hint) => {
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

impl Serialize for Float {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.value.into())
    }
}

//
// FloatWithSerializationMode
//

/// Adds [SerializationMode] support to [Float].
pub struct FloatWithSerializationMode<'a> {
    /// Wrapped value.
    pub float: &'a Float,

    /// Serialization mode.
    pub serialization_mode: &'a SerializationMode,
}

impl<'a> FloatWithSerializationMode<'a> {
    /// Constructor.
    pub fn new(float: &'a Float, serialization_mode: &'a SerializationMode) -> Self {
        Self { float, serialization_mode }
    }
}

impl<'a> Serialize for FloatWithSerializationMode<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.float.serialize_with_mode(serializer, self.serialization_mode)
    }
}

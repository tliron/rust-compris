use super::{super::normal::*, deserializer::*, errors::*};

//
// Value
//

impl Value {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<'de, T: serde::de::Deserialize<'de>>(&'de self) -> Result<T, DeserializationError> {
        T::deserialize(&mut Deserializer::new(self))
    }
}

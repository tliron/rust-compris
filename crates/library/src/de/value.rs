use super::{super::normal::*, deserializer::*, errors::*};

//
// Value
//

impl Value {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<'de, DeserializedT>(&'de self) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: serde::de::Deserialize<'de>,
    {
        DeserializedT::deserialize(&mut Deserializer::new(self))
    }
}

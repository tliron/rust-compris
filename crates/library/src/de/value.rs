use super::{super::normal::*, deserializer::*, errors::*};

use serde::de;

//
// Value
//

impl<AnnotatedT> Value<AnnotatedT> {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<'de, DeserializedT>(&'de self) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: de::Deserialize<'de>,
    {
        DeserializedT::deserialize(&mut Deserializer::new(self))
    }
}

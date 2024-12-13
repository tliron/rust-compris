use super::{super::*, errors::*};

use std::io::Read;

//
// Reader
//

impl read::Reader {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<ReadT, DeserializedT>(
        &mut self,
        reader: &mut ReadT,
    ) -> Result<DeserializedT, DeserializationError>
    where
        ReadT: Read,
        DeserializedT: serde::de::DeserializeOwned,
    {
        let value = self.read(reader)?;
        value.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_from_string<DeserializedT>(
        &mut self,
        string: &str,
    ) -> Result<DeserializedT, DeserializationError>
    where
        DeserializedT: serde::de::DeserializeOwned,
    {
        let value = self.read_from_string(string)?;
        value.deserialize()
    }
}

use super::{super::*, errors::*};

use std::io::Read;

//
// Reader
//

impl parse::Parser {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<ReadT, DeserializedT>(&mut self, reader: &mut ReadT) -> Result<DeserializedT, DeserializeError>
    where
        ReadT: Read,
        DeserializedT: serde::de::DeserializeOwned,
    {
        let value = self.parse(reader)?;
        value.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_from_string<DeserializedT>(&mut self, string: &str) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: serde::de::DeserializeOwned,
    {
        let value = self.parse_from_string(string)?;
        value.deserialize()
    }
}

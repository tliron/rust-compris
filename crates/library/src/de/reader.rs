use super::{super::*, errors::*};

use std::io::Read;

//
// Reader
//

impl read::Reader {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<R: Read, T: serde::de::DeserializeOwned>(
        &mut self,
        reader: &mut R,
    ) -> Result<T, DeserializationError> {
        let value = self.read(reader)?;
        value.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_from_string<T: serde::de::DeserializeOwned>(
        &mut self,
        string: &str,
    ) -> Result<T, DeserializationError> {
        let value = self.read_from_string(string)?;
        value.deserialize()
    }
}

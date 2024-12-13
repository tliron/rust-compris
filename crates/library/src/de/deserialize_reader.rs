use super::{super::*, errors::*};

use std::io::Read;

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
        self.deserialize(&mut string.as_bytes())
    }

    /// Deserialize. The provided value will be read into.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_into<'de, R: Read, T: serde::de::Deserialize<'de>>(
        &mut self,
        reader: &mut R,
        into_value: &'de mut Value,
    ) -> Result<T, DeserializationError> {
        *into_value = self.read(reader)?;
        into_value.deserialize()
    }
}

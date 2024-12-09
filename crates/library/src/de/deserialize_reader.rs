use super::{super::*, errors::*};

use std::io;

impl<R: io::Read> read::Reader<R> {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, DeserializationError> {
        let value = self.read()?;
        value.deserialize()
    }

    /// Read and deserialize. The provided value will be read into, so it can be
    /// initialized as [Value::Nothing].
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn read_and_deserialize<'de, T: serde::de::Deserialize<'de>>(
        &mut self,
        into_value: &'de mut Value,
    ) -> Result<T, DeserializationError> {
        *into_value = self.read()?;
        into_value.deserialize()
    }
}

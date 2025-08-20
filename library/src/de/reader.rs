use super::{
    super::{annotate::*, parse},
    errors::*,
};

use {serde::de, std::io};

impl parse::Parser {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_reader<ReadT, DeserializedT, AnnotatedT>(
        &mut self,
        reader: &mut ReadT,
    ) -> Result<DeserializedT, DeserializeError>
    where
        ReadT: io::Read,
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: Annotated + Clone + Default,
    {
        let variant = self.parse_reader::<_, AnnotatedT>(reader)?;
        variant.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_string<DeserializedT, AnnotatedT>(
        &mut self,
        string: &str,
    ) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: Annotated + Clone + Default,
    {
        let variant = self.parse_string::<AnnotatedT>(string)?;
        variant.deserialize()
    }
}

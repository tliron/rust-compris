use super::{
    super::{annotate::*, parse},
    errors::*,
};

use {serde::de, std::io};

//
// Reader
//

impl parse::Parser {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<ReadT, DeserializedT, AnnotatedT>(
        &mut self,
        reader: &mut ReadT,
    ) -> Result<DeserializedT, DeserializeError>
    where
        ReadT: io::Read,
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: Annotated + Clone + Default,
    {
        let variant = self.parse::<_, AnnotatedT>(reader)?;
        variant.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_from_string<DeserializedT, AnnotatedT>(
        &mut self,
        string: &str,
    ) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: Annotated + Clone + Default,
    {
        let variant = self.parse_from_string::<AnnotatedT>(string)?;
        variant.deserialize()
    }
}

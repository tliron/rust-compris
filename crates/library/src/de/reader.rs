use super::{
    super::{annotation::*, parse},
    errors::*,
};

use {serde::de, std::io::Read};

//
// Reader
//

impl parse::Parser {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<ReadT, DeserializedT, AnnotationsT>(
        &mut self,
        reader: &mut ReadT,
    ) -> Result<DeserializedT, DeserializeError>
    where
        ReadT: Read,
        DeserializedT: de::DeserializeOwned,
        AnnotationsT: Annotated + Clone + Default,
    {
        let value = self.parse::<_, AnnotationsT>(reader)?;
        value.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_from_string<DeserializedT, AnnotationsT>(
        &mut self,
        string: &str,
    ) -> Result<DeserializedT, DeserializeError>
    where
        DeserializedT: de::DeserializeOwned,
        AnnotationsT: Annotated + Clone + Default,
    {
        let value = self.parse_from_string::<AnnotationsT>(string)?;
        value.deserialize()
    }
}

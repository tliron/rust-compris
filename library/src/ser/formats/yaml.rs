use super::super::{errors::*, serializer::*};

use {serde::Serialize, std::io};

impl Serializer {
    /// Serializes the provided value to the writer as YAML.
    pub fn write_yaml<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: io::Write,
        SerializableT: Serialize + ?Sized,
    {
        // Broken for complex keys
        let config = serde_yml::ser::SerializerConfig { tag_unit_variants: true };
        let mut serializer = serde_yml::Serializer::new_with_config(writer, config);
        Ok(value.serialize(&mut serializer)?)
    }
}

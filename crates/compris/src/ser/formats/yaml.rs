use super::super::errors::*;

use {serde::*, std::io::Write};

impl<W: Write> super::super::serializer::Serializer<W> {
    pub fn write_yaml<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), WriteError> {
        // Broken for complex keys
        let config = serde_yml::ser::SerializerConfig { tag_unit_variants: true };
        let mut serializer = serde_yml::Serializer::new_with_config(self.writer.by_ref(), config);
        Ok(value.serialize(&mut serializer)?)
    }
}

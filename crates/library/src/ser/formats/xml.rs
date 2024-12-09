use super::super::{Serializer as ComprisSerializer, *};

use {serde::*, std::io::Write};

impl<W: Write> ComprisSerializer<W> {
    // Broken :(
    // Write out own using https://docs.rs/quick-xml/latest/quick_xml/
    /// Serializes the provided value to the writer as XML.
    pub fn write_xml<V: Serialize>(&mut self, value: &V) -> Result<(), SerializationError> {
        // Note: serde_xml_rs requires value to be Sized
        serde_xml_rs::to_writer(self.writer.by_ref(), value)?;

        if self.pretty {
            self.write_newline()
        } else {
            Ok(())
        }
    }
}

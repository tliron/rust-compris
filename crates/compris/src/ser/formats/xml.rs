use super::super::errors::*;

use {serde::*, std::io::Write};

impl<W: Write> super::super::serializer::Serializer<W> {
    // Broken :(
    // Write out own using https://docs.rs/quick-xml/latest/quick_xml/
    pub fn write_xml<V: Serialize>(&mut self, value: &V) -> Result<(), WriteError> {
        // Note: serde_xml_rs requires value to be Sized
        serde_xml_rs::to_writer(self.writer.by_ref(), value)?;

        if self.pretty {
            self.write_newline()
        } else {
            Ok(())
        }
    }
}

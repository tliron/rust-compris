use super::super::*;

use {serde::Serialize, std::io::Write};

//
// Serializer
//

impl Serializer {
    // Broken :(
    // Write out own using https://docs.rs/quick-xml/latest/quick_xml/
    /// Serializes the provided value to the writer as XML.
    pub fn write_xml<W: Write, V: Serialize + Sized>(
        &self,
        value: &V,
        writer: &mut W,
    ) -> Result<(), SerializationError> {
        // Note: serde_xml_rs requires value to be Sized
        serde_xml_rs::to_writer(writer.by_ref(), value)?;

        if self.pretty {
            Self::write_newline(writer)
        } else {
            Ok(())
        }
    }
}

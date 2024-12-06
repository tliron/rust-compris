use super::super::errors::*;

use {serde::*, std::io::Write};

impl<W: Write> super::super::serializer::Serializer<W> {
    pub fn write_message_pack<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), WriteError> {
        fn write<V: Serialize + ?Sized>(value: &V, writer: &mut impl Write) -> Result<(), WriteError> {
            Ok(rmp_serde::encode::write(writer, value)?)
        }

        if self.base64 {
            write(value, &mut self.base64_writer())?;
        } else {
            write(value, self.writer.by_ref())?;
        }

        if self.pretty {
            self.write_newline()
        } else {
            Ok(())
        }
    }
}

use super::super::{Serializer as ComprisSerializer, *};

use {serde::*, std::io::Write};

impl<W: Write> ComprisSerializer<W> {
    /// Serializes the provided value to the writer as MessagePack.
    ///
    /// Is affected by [ComprisSerializer::base64].
    pub fn write_message_pack<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), SerializationError> {
        fn write<V: Serialize + ?Sized>(value: &V, writer: &mut impl Write) -> Result<(), SerializationError> {
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

use super::super::{Serializer as ComprisSerializer, *};

use {serde::*, std::io::Write};

impl ComprisSerializer {
    /// Serializes the provided value to the writer as MessagePack.
    ///
    /// Is affected by [ComprisSerializer::base64].
    pub fn write_message_pack<W: Write, V: Serialize + ?Sized>(
        &self,
        value: &V,
        writer: &mut W,
    ) -> Result<(), SerializationError> {
        fn write<W: Write, V: Serialize + ?Sized>(value: &V, writer: &mut W) -> Result<(), SerializationError> {
            Ok(rmp_serde::encode::write(writer, value)?)
        }

        if self.base64 {
            write(value, &mut Self::base64_writer(writer))?;
        } else {
            write(value, writer)?;
        }

        if self.pretty {
            Self::write_newline(writer)
        } else {
            Ok(())
        }
    }
}

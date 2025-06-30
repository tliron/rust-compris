use super::super::{errors::*, serializer::*};

use {serde::Serialize, std::io};

impl Serializer {
    /// Serializes the provided value to the writer as MessagePack.
    ///
    /// Is affected by [Serializer::base64](super::super::Serializer::base64).
    pub fn write_message_pack<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: io::Write,
        SerializableT: Serialize + ?Sized,
    {
        fn write<WriteT, SerializableT>(value: &SerializableT, writer: &mut WriteT) -> Result<(), SerializeError>
        where
            WriteT: io::Write,
            SerializableT: Serialize + ?Sized,
        {
            Ok(rmp_serde::encode::write(writer, value)?)
        }

        if self.base64 {
            write(value, &mut Self::base64_writer(writer))?;
        } else {
            write(value, writer)?;
        }

        if self.pretty { Self::write_newline(writer) } else { Ok(()) }
    }
}

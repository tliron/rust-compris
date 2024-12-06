use super::{super::*, errors::*};

use {
    serde::*,
    std::{
        io::{BufWriter, Write},
        string,
    },
};

//
// Serializer
//

impl<W: Write> super::serializer::Serializer<W> {
    pub fn write<V: Serialize>(&mut self, value: &V) -> Result<(), WriteError> {
        match self.format {
            #[cfg(feature = "yaml")]
            Format::YAML => self.write_yaml(value),

            #[cfg(feature = "json")]
            Format::JSON => self.write_json(value),

            #[cfg(feature = "xml")]
            Format::XML => self.write_xml(value),

            #[cfg(feature = "cbor")]
            Format::CBOR => self.write_cbor(value),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.write_message_pack(value),

            _ => Err(WriteError::UnsupportedFormat(self.format.clone())),
        }
    }

    pub fn stringify<V: Serialize>(&mut self, value: &V) -> Result<string::String, WriteError> {
        let mut serializer = super::serializer::Serializer::new(BufWriter::new(Vec::new()))
            .with_format(self.format.clone())
            .with_pretty(self.pretty)
            .with_indent(self.indent)
            .with_strict(self.strict)
            .with_base64(true);

        match serializer.write(value) {
            Ok(_) => Ok(string::String::from_utf8(serializer.writer.buffer().into())?),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn write_newline(&mut self) -> Result<(), WriteError> {
        self.writer.write("\n".as_bytes())?;
        Ok(())
    }

    pub(crate) fn base64_writer(&mut self) -> base64::write::EncoderWriter<base64::engine::GeneralPurpose, &mut W> {
        base64::write::EncoderWriter::new(self.writer.by_ref(), &base64::engine::general_purpose::STANDARD)
    }
}

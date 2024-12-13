use super::{super::*, errors::*};

use {
    serde::*,
    std::{
        fs::*,
        io::{stdout, BufWriter, Write},
        path,
    },
};

//
// Serializer
//

/// General-purpose serde serializer supporting various formats.
#[derive(Clone)]
pub struct Serializer {
    /// Format.
    pub format: Format,

    /// Pretty output (for YAML, JSON, and XML). Defaults to false.
    pub pretty: bool,

    /// Indent for pretty output (for YAML, JSON, and XML). Defaults to 2.
    pub indent: u64,

    /// Base64 output (for CBOR and MessagePack). Defaults to false.
    pub base64: bool,
}

impl Serializer {
    /// Constructor.
    pub fn new(format: Format) -> Self {
        Self { format, pretty: false, indent: 2, base64: false }
    }

    /// Set format.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Set pretty output (for YAML, JSON, and XML).
    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    /// Set indent for pretty output (for YAML, JSON, and XML).
    pub fn with_indent(mut self, indent: u64) -> Self {
        self.indent = indent;
        self
    }

    /// Set Base64 output (for CBOR and MessagePack).
    pub fn with_base64(mut self, base64: bool) -> Self {
        self.base64 = base64;
        self
    }

    /// Serializes the provided value to the writer according to [Serializer::format].
    pub fn write<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializationError>
    where
        WriteT: Write,
        SerializableT: Serialize,
    {
        match self.format {
            #[cfg(feature = "yaml")]
            Format::YAML => self.write_yaml(value, writer),

            #[cfg(feature = "json")]
            Format::JSON | Format::XJSON => self.write_json(value, writer),

            #[cfg(feature = "xml")]
            Format::XML => self.write_xml(value, writer),

            #[cfg(feature = "cbor")]
            Format::CBOR => self.write_cbor(value, writer),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.write_message_pack(value, writer),

            #[cfg(not(all(
                feature = "yaml",
                feature = "json",
                feature = "xml",
                feature = "cbor",
                feature = "messagepack"
            )))]
            _ => Err(SerializationError::UnsupportedFormat(self.format.clone())),
        }
    }

    /// Serializes the provided value to the file according to [Serializer::format].
    pub fn write_to_file<SerializableT>(
        &self,
        value: &SerializableT,
        path: &path::Path,
    ) -> Result<(), SerializationError>
    where
        SerializableT: Serialize,
    {
        self.write(value, &mut BufWriter::new(File::create(path)?))
    }

    /// Serializes the provided value to the file according to [Serializer::format].
    pub fn write_to_stdout<SerializableT>(&self, value: &SerializableT) -> Result<(), SerializationError>
    where
        SerializableT: Serialize,
    {
        self.write(value, &mut stdout())
    }

    /// Convenience function to serialize to a string.
    ///
    /// See [Serializer::write].
    pub fn stringify<SerializableT>(&self, value: &SerializableT) -> Result<String, SerializationError>
    where
        SerializableT: Serialize,
    {
        let serializer =
            Serializer::new(self.format.clone()).with_pretty(self.pretty).with_indent(self.indent).with_base64(true);

        let mut writer = BufWriter::new(Vec::new());
        match serializer.write(value, &mut writer) {
            Ok(_) => Ok(String::from_utf8(writer.buffer().into())?),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn write_newline<WriteT>(writer: &mut WriteT) -> Result<(), SerializationError>
    where
        WriteT: Write,
    {
        writer.write("\n".as_bytes())?;
        Ok(())
    }

    pub(crate) fn base64_writer<'a, WriteT>(
        writer: &'a mut WriteT,
    ) -> base64::write::EncoderWriter<'a, base64::engine::GeneralPurpose, &'a mut WriteT>
    where
        WriteT: Write,
    {
        base64::write::EncoderWriter::new(writer, &base64::engine::general_purpose::STANDARD)
    }
}

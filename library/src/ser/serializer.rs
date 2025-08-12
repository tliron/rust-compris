use super::{
    super::{annotate::*, normal::*, *},
    errors::*,
    modal::*,
    mode::*,
};

use {
    kutil::std::zerocopy::*,
    serde::*,
    std::{
        fs::*,
        io::{BufWriter, Write, stdout},
        path,
    },
};

const STRINGIFY_BUFFER_CAPACITY: usize = 1024;

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

    /// Serializes the provided value to the writer according to [Serializer::format](Serializer).
    pub fn write<WriteT, SerializableT>(&self, value: &SerializableT, writer: &mut WriteT) -> Result<(), SerializeError>
    where
        WriteT: Write,
        SerializableT: Serialize,
    {
        match self.format {
            #[cfg(feature = "cbor")]
            Format::CBOR => self.write_cbor(value, writer),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.write_message_pack(value, writer),

            #[cfg(feature = "yaml")]
            Format::YAML => self.write_yaml(value, writer),

            #[cfg(feature = "json")]
            Format::JSON | Format::XJSON => self.write_json(value, writer),

            #[cfg(feature = "xml")]
            Format::XML => self.write_xml(value, writer),

            #[cfg(not(all(
                feature = "cbor",
                feature = "messagepack",
                feature = "yaml",
                feature = "json",
                feature = "xml",
            )))]
            _ => Err(SerializeError::UnsupportedFormat(self.format.clone())),
        }
    }

    /// Serializes the provided value to the writer according to [Serializer::format](Serializer).
    pub fn write_modal<WriteT, AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: Write,
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.write(&value, writer)
    }

    /// Serializes the provided value to the file according to [Serializer::format](Serializer).
    pub fn write_to_file<SerializableT, PathT>(&self, value: &SerializableT, path: PathT) -> Result<(), SerializeError>
    where
        SerializableT: Serialize,
        PathT: AsRef<path::Path>,
    {
        self.write(value, &mut BufWriter::new(File::create(path)?))
    }

    /// Serializes the provided value to the file according to [Serializer::format](Serializer).
    pub fn write_to_file_modal<AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
        path: &path::Path,
    ) -> Result<(), SerializeError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.write_to_file(&value, path)
    }

    /// Serializes the provided value to [stdout] according to [Serializer::format](Serializer).
    pub fn print<SerializableT>(&self, value: &SerializableT) -> Result<(), SerializeError>
    where
        SerializableT: Serialize,
    {
        self.write(value, &mut stdout())
    }

    /// Serializes the provided value to [stdout] according to [Serializer::format](Serializer).
    pub fn print_modal<AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
    ) -> Result<(), SerializeError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.print(&value)
    }

    /// Convenience function to serialize to a string.
    ///
    /// See [Serializer::write].
    pub fn stringify<SerializableT>(&self, value: &SerializableT) -> Result<ByteString, SerializeError>
    where
        SerializableT: Serialize,
    {
        let serializer =
            Serializer::new(self.format.clone()).with_pretty(self.pretty).with_indent(self.indent).with_base64(true);

        let mut writer = Vec::with_capacity(STRINGIFY_BUFFER_CAPACITY);
        match serializer.write(value, &mut writer) {
            Ok(_) => Ok(ByteString::try_from(writer)?),
            Err(error) => Err(error),
        }
    }

    /// Convenience function to serialize to a string.
    ///
    /// See [Serializer::write].
    pub fn stringify_modal<AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
    ) -> Result<ByteString, SerializeError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.stringify(&value)
    }

    // Utils

    #[allow(dead_code)]
    pub(crate) fn write_newline<WriteT>(writer: &mut WriteT) -> Result<(), SerializeError>
    where
        WriteT: Write,
    {
        writer.write("\n".as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn base64_writer<'own, WriteT>(
        writer: &'own mut WriteT,
    ) -> base64::write::EncoderWriter<'own, base64::engine::GeneralPurpose, &'own mut WriteT>
    where
        WriteT: Write,
    {
        base64::write::EncoderWriter::new(writer, &base64::engine::general_purpose::STANDARD)
    }
}

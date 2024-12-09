use super::{super::*, errors::*};

use {
    serde::*,
    std::{
        fs::*,
        io::{self, stdout, BufWriter, Stdout, Write},
        path::*,
        string::String as StdString,
    },
};

//
// Serializer
//

/// General-purpose serde serializer supporting various formats.
#[derive(Clone)]
pub struct Serializer<W: Write + Sized> {
    /// Writer.
    pub writer: W,

    /// Format.
    pub format: Format,

    /// Pretty output (for YAML, JSON, and XML). Defaults to false.
    pub pretty: bool,

    /// Indent for pretty output (for YAML, JSON, and XML). Defaults to 2.
    pub indent: u64,

    /// Strict output (for YAML only). Defaults to false.
    pub strict: bool,

    /// Base64 output (for CBOR and MessagePack). Defaults to false.
    pub base64: bool,
}

impl Serializer<Stdout> {
    /// Constructor.
    pub fn new_for_stdout() -> Serializer<Stdout> {
        Serializer::new(stdout())
    }
}

impl<W: Write> Serializer<W> {
    /// Constructor.
    pub fn new(writer: W) -> Self {
        Self { writer, format: Format::default(), pretty: false, indent: 2, strict: false, base64: false }
    }

    /// Constructor.
    pub fn new_for_file(path: &Path) -> Result<Serializer<BufWriter<File>>, io::Error> {
        Ok(Serializer::new(BufWriter::new(File::create(path)?)))
    }

    /// Set writer.
    pub fn with_writer(mut self, writer: W) -> Self {
        self.writer = writer;
        self
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

    /// Set strict output (for YAML only).
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Set Base64 output (for CBOR and MessagePack).
    pub fn with_base64(mut self, base64: bool) -> Self {
        self.base64 = base64;
        self
    }

    /// Serializes the provided value to the writer according to [Serializer::format].
    pub fn write<V: Serialize>(&mut self, value: &V) -> Result<(), SerializationError> {
        match self.format {
            #[cfg(feature = "yaml")]
            Format::YAML => self.write_yaml(value),

            #[cfg(feature = "json")]
            Format::JSON | Format::XJSON => self.write_json(value),

            #[cfg(feature = "xml")]
            Format::XML => self.write_xml(value),

            #[cfg(feature = "cbor")]
            Format::CBOR => self.write_cbor(value),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.write_message_pack(value),

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

    /// Convenience function to serialize to a string.
    ///
    /// See [Serializer::write].
    pub fn stringify<V: Serialize>(&mut self, value: &V) -> Result<StdString, SerializationError> {
        let mut serializer = Serializer::new(BufWriter::new(Vec::new()))
            .with_format(self.format.clone())
            .with_pretty(self.pretty)
            .with_indent(self.indent)
            .with_strict(self.strict)
            .with_base64(true);

        match serializer.write(value) {
            Ok(_) => Ok(StdString::from_utf8(serializer.writer.buffer().into())?),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn write_newline(&mut self) -> Result<(), SerializationError> {
        self.writer.write("\n".as_bytes())?;
        Ok(())
    }

    pub(crate) fn base64_writer(&mut self) -> base64::write::EncoderWriter<base64::engine::GeneralPurpose, &mut W> {
        base64::write::EncoderWriter::new(self.writer.by_ref(), &base64::engine::general_purpose::STANDARD)
    }
}

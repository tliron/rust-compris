use super::super::*;

use std::{
    fs::*,
    io::{self, stdout, BufWriter, Stdout, Write},
    path::*,
};

//
// Serializer
//

#[derive(Clone)]
pub struct Serializer<W: Write + Sized> {
    pub writer: W,

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
    pub fn new_for_stdout() -> Serializer<Stdout> {
        Serializer::new(stdout())
    }
}

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, format: Format::default(), pretty: false, indent: 2, strict: false, base64: false }
    }

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
}

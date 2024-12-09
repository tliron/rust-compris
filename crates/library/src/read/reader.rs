use super::{super::*, errors::*};

use std::io::Read;

//
// Reader
//

/// Reads from various formats into normal value types.
pub struct Reader<R: Read> {
    /// Reader.
    pub reader: R,

    /// Format.
    pub format: Format,

    /// Try to parse numbers as integers (for JSON only). Defaults to false.
    pub try_integers: bool,

    /// Try to parse numbers as unsigned integers (for YAML and JSON). Defaults to false.
    pub try_unsigned_integers: bool,

    /// Allow legacy words (for YAML only). Defaults to false.
    pub allow_legacy_words: bool,

    /// Allow legacy types (for YAML only). Defaults to false.
    pub allow_legacy_types: bool,

    /// Decode Base64 (for CBOR and MessagePack only). Defaults to false.
    pub base64: bool,
}

impl<R: Read> Reader<R> {
    /// Constructor.
    pub fn new(reader: R, format: Format) -> Self {
        Self {
            reader,
            format,
            try_integers: false,
            try_unsigned_integers: false,
            allow_legacy_words: false,
            allow_legacy_types: false,
            base64: false,
        }
    }

    /// Set reader.
    pub fn with_reader(mut self, reader: R) -> Self {
        self.reader = reader;
        self
    }

    /// Set format.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Set whether to try to parse numbers as integers (for JSON only).
    ///
    /// False sets [Reader::try_unsigned_integers] to false.
    pub fn with_try_integers(mut self, allow_integers: bool) -> Self {
        self.try_integers = allow_integers;
        if !allow_integers {
            self.try_unsigned_integers = false;
        }
        self
    }

    /// Set whether to try to parse numbers as unsigned integers (for YAML and JSON).
    ///
    /// True sets [Reader::try_integers] to false.
    pub fn with_try_unsigned_integers(mut self, allow_unsigned_integers: bool) -> Self {
        self.try_unsigned_integers = allow_unsigned_integers;
        if allow_unsigned_integers {
            self.try_integers = true;
        }
        self
    }

    /// Set whether to allow legacy words (for YAML only).
    pub fn with_allow_legacy_words(mut self, allow_legacy_words: bool) -> Self {
        self.allow_legacy_words = allow_legacy_words;
        self
    }

    /// Set whether to allow legacy types (for YAML only).
    pub fn with_allow_legacy_types(mut self, allow_legacy_types: bool) -> Self {
        self.allow_legacy_types = allow_legacy_types;
        self
    }

    /// Set whether to decode Base64 (for CBOR and MessagePack only).
    pub fn with_base64(mut self, base64: bool) -> Self {
        self.base64 = base64;
        self
    }

    /// Reads into a normal value according to [Reader::format].
    pub fn read(&mut self) -> Result<Value, ReadError> {
        match &self.format {
            #[cfg(feature = "yaml")]
            Format::YAML => self.read_yaml(),

            #[cfg(feature = "json")]
            Format::JSON => self.read_json(),

            #[cfg(feature = "json")]
            Format::XJSON => self.read_xjson(),

            #[cfg(feature = "xml")]
            Format::XML => todo!(),

            #[cfg(feature = "cbor")]
            Format::CBOR => self.read_cbor(),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.read_message_pack(),

            #[cfg(not(all(
                feature = "yaml",
                feature = "json",
                feature = "xml",
                feature = "cbor",
                feature = "messagepack"
            )))]
            _ => Err(ReadError::UnsupportedFormat(self.format.clone())),
        }
    }
}

//
// StringReader
//

/// String reader.
pub trait StringReader<'a> {
    /// Constructor.
    fn new_for_string(string: &'a str, format: Format) -> Self;
}

impl<'a> StringReader<'a> for Reader<&'a [u8]> {
    fn new_for_string(string: &'a str, format: Format) -> Self {
        Self::new(string.as_bytes(), format)
    }
}

use super::{super::*, errors::*};

use std::io::Read;

//
// Reader
//

pub struct Reader<R: Read> {
    pub reader: R,

    pub format: Format,

    /// Allow integers (for JSON only). Defaults to false.
    pub allow_integers: bool,

    /// Allow unsigned integers (for YAML and JSON). Defaults to false.
    pub allow_unsigned_integers: bool,

    /// Allow legacy syntax (for YAML only). Defaults to false.
    pub allow_legacy: bool,

    /// Decode Base64 (for CBOR and MessagePack only). Defaults to false.
    pub base64: bool,
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R, format: Format) -> Self {
        Self {
            reader,
            format,
            allow_integers: false,
            allow_unsigned_integers: false,
            allow_legacy: false,
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

    /// Set whether to allow integers (for JSON only).
    ///
    /// False sets [Reader::allow_unsigned_integers] to false.
    pub fn with_allow_integers(mut self, allow_integers: bool) -> Self {
        self.allow_integers = allow_integers;
        if !allow_integers {
            self.allow_unsigned_integers = false;
        }
        self
    }

    /// Set whether to allow unsigned integers (for YAML and JSON).
    ///
    /// True sets [Reader::allow_integers] to false.
    pub fn with_allow_unsigned_integers(mut self, allow_unsigned_integers: bool) -> Self {
        self.allow_unsigned_integers = allow_unsigned_integers;
        if allow_unsigned_integers {
            self.allow_integers = true;
        }
        self
    }

    /// Set whether to allow legacy syntax (for YAML only).
    pub fn with_allow_legacy(mut self, legacy: bool) -> Self {
        self.allow_legacy = legacy;
        self
    }

    /// Set whether to decode Base64 (for CBOR and MessagePack only).
    pub fn with_base64(mut self, base64: bool) -> Self {
        self.base64 = base64;
        self
    }

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

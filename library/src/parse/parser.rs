use super::{
    super::{annotate::*, format::*, normal::*},
    error::*,
};

use {kutil::std::zerocopy::*, std::io};

//
// Parser
//

/// Parses various formats into [Variant].
pub struct Parser {
    /// Format.
    pub format: Format,

    /// Source.
    pub source: Option<ByteString>,

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

impl Parser {
    /// Constructor.
    pub fn new(format: Format) -> Self {
        Self {
            format,
            source: None,
            try_integers: false,
            try_unsigned_integers: false,
            allow_legacy_words: false,
            allow_legacy_types: false,
            base64: false,
        }
    }

    /// Set format.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Set source.
    pub fn with_source(mut self, source: ByteString) -> Self {
        self.source = Some(source);
        self
    }

    /// Set whether to try to parse numbers as integers (for JSON only).
    ///
    /// False sets [Parser::try_unsigned_integers] to false.
    pub fn with_try_integers(mut self, allow_integers: bool) -> Self {
        self.try_integers = allow_integers;
        if !allow_integers {
            self.try_unsigned_integers = false;
        }
        self
    }

    /// Set whether to try to parse numbers as unsigned integers (for YAML and JSON).
    ///
    /// True sets [Parser::try_integers] to false.
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

    /// Parses into a [Variant] according to [Parser::format].
    pub fn parse<ReadT, AnnotatedT>(&self, reader: &mut ReadT) -> Result<Variant<AnnotatedT>, ParseError>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
    {
        match &self.format {
            #[cfg(feature = "cbor")]
            Format::CBOR => self.parse_cbor(reader),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.parse_message_pack(reader),

            #[cfg(feature = "yaml")]
            Format::YAML => self.parse_yaml(reader),

            #[cfg(feature = "json")]
            Format::JSON => self.parse_json(reader),

            #[cfg(feature = "json")]
            Format::XJSON => self.parse_xjson(reader),

            #[cfg(feature = "xml")]
            Format::XML => todo!(),

            #[cfg(not(all(
                feature = "cbor",
                feature = "messagepack",
                feature = "yaml",
                feature = "json",
                feature = "xml",
            )))]
            _ => Err(ParseError::UnsupportedFormat(self.format.clone())),
        }
    }

    /// Parses into a [Variant] according to [Parser::format].
    pub fn parse_from_string<AnnotatedT>(&self, string: &str) -> Result<Variant<AnnotatedT>, ParseError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        self.parse(&mut string.as_bytes())
    }
}

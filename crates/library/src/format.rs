use {
    std::{convert::*, fmt},
    thiserror::*,
};

//
// Format
//

/// CPS format.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Format {
    /// CBOR.
    CBOR,

    /// MessagePack.
    MessagePack,

    /// YAML.
    #[default]
    YAML,

    /// JSON.
    JSON,

    /// XJSON.
    XJSON,

    /// XML.
    XML,
}

impl Format {
    /// String identifier for the format.
    pub fn get_identifier(&self) -> &'static str {
        match self {
            Self::CBOR => "cbor",
            Self::MessagePack => "messagepack",
            Self::YAML => "yaml",
            Self::JSON => "json",
            Self::XJSON => "xjson",
            Self::XML => "xml",
        }
    }

    /// Whether or not this is a binary format (CBOR or MessagePack).
    pub fn is_binary(&self) -> bool {
        match self {
            Self::CBOR | Self::MessagePack => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for Format {
    type Error = UnknownFormatError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match &*string.to_lowercase() {
            "cbor" => Ok(Self::CBOR),
            "messagepack" => Ok(Self::MessagePack),
            "yaml" => Ok(Self::YAML),
            "json" => Ok(Self::JSON),
            "xjson" => Ok(Self::XJSON),
            "xml" => Ok(Self::XML),
            _ => Err(UnknownFormatError::new(string)),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.get_identifier(), formatter)
    }
}

//
// UnknownFormatError
//

/// Uknown format error.
#[derive(Debug, Error)]
pub struct UnknownFormatError(String);

impl UnknownFormatError {
    /// Constructor.
    pub fn new(format: &str) -> Self {
        Self(format.into())
    }
}

impl fmt::Display for UnknownFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

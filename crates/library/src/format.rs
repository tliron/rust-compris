use {
    std::{convert::*, fmt},
    thiserror::*,
};

//
// Format
//

/// CPS format.
#[derive(Default, PartialEq, Clone, Debug)]
pub enum Format {
    /// YAML.
    #[default]
    YAML,

    /// JSON.
    JSON,

    /// XJSON.
    XJSON,

    /// XML.
    XML,

    /// CBOR.
    CBOR,

    /// MessagePack.
    MessagePack,
}

impl Format {
    /// String identifier for the format.
    pub fn get_identifier(&self) -> &'static str {
        match self {
            Self::YAML => "yaml",
            Self::JSON => "json",
            Self::XJSON => "xjson",
            Self::XML => "xml",
            Self::CBOR => "cbor",
            Self::MessagePack => "messagepack",
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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &*value.to_lowercase() {
            "yaml" => Ok(Self::YAML),
            "json" => Ok(Self::JSON),
            "xjson" => Ok(Self::XJSON),
            "xml" => Ok(Self::XML),
            "cbor" => Ok(Self::CBOR),
            "messagepack" => Ok(Self::MessagePack),
            _ => Err(UnknownFormatError::new(value)),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get_identifier().fmt(formatter)
    }
}

//
// UnknownFormatError
//

/// Uknown format error.
#[derive(Error, Debug)]
pub struct UnknownFormatError(String);

impl UnknownFormatError {
    /// Constructor.
    pub fn new(format: &str) -> Self {
        Self(format.into())
    }
}

impl fmt::Display for UnknownFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

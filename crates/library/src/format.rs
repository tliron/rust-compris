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
    /// Whether or not this is a binary format (CBOR or MessagePack).
    pub fn is_binary(&self) -> bool {
        match self {
            Format::CBOR | Format::MessagePack => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for Format {
    type Error = UnknownFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &*value.to_lowercase() {
            "yaml" => Ok(Format::YAML),
            "json" => Ok(Format::JSON),
            "xjson" => Ok(Format::XJSON),
            "xml" => Ok(Format::XML),
            "cbor" => Ok(Format::CBOR),
            "messagepack" => Ok(Format::MessagePack),
            _ => Err(UnknownFormatError(value.into())),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::YAML => "yaml".fmt(formatter),
            Format::JSON => "json".fmt(formatter),
            Format::XJSON => "xjson".fmt(formatter),
            Format::XML => "xml".fmt(formatter),
            Format::CBOR => "cbor".fmt(formatter),
            Format::MessagePack => "messagepack".fmt(formatter),
        }
    }
}

//
// UnknownFormatError
//

/// Uknown format error.
#[derive(Error, Debug)]
pub struct UnknownFormatError(pub String);

impl fmt::Display for UnknownFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

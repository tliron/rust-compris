use {
    kutil_std::message_error,
    std::{convert::*, fmt, str::*},
};

//
// Format
//

/// CPS format.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub fn identifier(&self) -> &'static str {
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

impl Into<&'static str> for Format {
    fn into(self) -> &'static str {
        self.identifier()
    }
}

impl FromStr for Format {
    type Err = UnknownFormatError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation.to_lowercase().as_str() {
            "cbor" => Ok(Self::CBOR),
            "messagepack" => Ok(Self::MessagePack),
            "yaml" => Ok(Self::YAML),
            "json" => Ok(Self::JSON),
            "xjson" => Ok(Self::XJSON),
            "xml" => Ok(Self::XML),
            _ => Err(representation.into()),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.identifier(), formatter)
    }
}

//
// UnknownFormatError
//

message_error!(UnknownFormatError, "unknown format");

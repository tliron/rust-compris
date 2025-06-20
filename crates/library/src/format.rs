use {
    kutil_std::{message_error, *},
    std::convert::*,
};

//
// Format
//

/// CPS format.
#[derive(Clone, Copy, Debug, Default, Display, FromStr, PartialEq)]
#[display(lowercase)]
#[from_str(lowercase, error = UnknownFormatError)]
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
    #[strings("xjson")]
    XJSON,

    /// XML.
    XML,
}

impl Format {
    /// Whether or not this is a binary format (CBOR or MessagePack).
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::CBOR | Self::MessagePack)
    }
}

//
// UnknownFormatError
//

message_error!(UnknownFormatError, "unknown format");

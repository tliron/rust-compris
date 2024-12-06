use std::convert::*;

//
// Format
//

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Format {
    #[default]
    YAML,
    JSON,
    XJSON,
    XML,
    CBOR,
    MessagePack,
}

impl Format {
    pub fn is_bytes(&self) -> bool {
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

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::YAML => "yaml".into(),
            Format::JSON => "json".into(),
            Format::XJSON => "xjson".into(),
            Format::XML => "xml".into(),
            Format::CBOR => "cbor".into(),
            Format::MessagePack => "messagepack".into(),
        }
    }
}

//
// UnknownFormatError
//

#[derive(Debug)]
pub struct UnknownFormatError(pub String);

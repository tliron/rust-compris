use thiserror::*;

//
// CborError
//

use std::{fmt, string};

/// CBOR write error.
#[derive(Debug, Error)]
pub struct CborWriteError {
    borc: Option<borc::errors::EncodeError>,
    custom: string::String,
}

impl fmt::Display for CborWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.borc {
            Some(borc) => write!(formatter, "{:?}", borc),
            None => fmt::Display::fmt(&self.custom, formatter),
        }
    }
}

impl serde::ser::Error for CborWriteError {
    fn custom<DisplayableT>(msg: DisplayableT) -> Self
    where
        DisplayableT: fmt::Display,
    {
        Self { borc: None, custom: format!("{}", msg) }
    }
}

// Conversions

#[cfg(feature = "cbor")]
impl From<borc::errors::EncodeError> for CborWriteError {
    fn from(encode_error: borc::errors::EncodeError) -> Self {
        Self { borc: Some(encode_error), custom: Default::default() }
    }
}

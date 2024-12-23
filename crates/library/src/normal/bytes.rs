use super::{meta::*, normal::*};

use {
    base64::{prelude::*, *},
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Bytes
//

/// Normal bytes value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Bytes {
    /// Actual value.
    pub value: Vec<u8>,

    /// Metadata.
    pub meta: Meta,
}

impl Bytes {
    /// Constructor.
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }

    /// Constructor.
    pub fn new_from_base64(value: &str) -> Result<Self, DecodeError> {
        let bytes = BASE64_STANDARD.decode(value)?;
        Ok(Self::new(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

impl Normal for Bytes {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        self.to_base64()
    }
}

impl Debuggable for Bytes {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", format!("{} bytes", self.value.len()).style(styles.plain))?;
        if let Some(coordinates) = &self.meta.coordinates {
            write!(writer, " ")?;
            coordinates.write_debug_representation(writer, prefix, styles)?;
        }
        Ok(())
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}b64", self.to_base64())
    }
}

// Delegated

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Bytes {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

// Conversion

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes::new(bytes)
    }
}

impl From<&[u8]> for Bytes {
    fn from(bytes: &[u8]) -> Self {
        Bytes::new(bytes)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.value
    }
}

impl<'a> From<&'a Bytes> for &'a [u8] {
    fn from(bytes: &'a Bytes) -> Self {
        &bytes.value
    }
}

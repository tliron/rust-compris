use super::super::meta::*;

use {
    base64::{prelude::*, *},
    duplicate::*,
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Bytes
//

/// Normal bytes value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Bytes {
    /// Actual value.
    pub value: Vec<u8>,

    /// Metadata.
    pub meta: Meta,
}

impl Bytes {
    /// Constructor.
    pub fn new<BytesT>(bytes: BytesT) -> Self
    where
        BytesT: Into<Vec<u8>>,
    {
        Self { value: bytes.into(), ..Default::default() }
    }

    /// Constructor.
    pub fn new_from_base64(base64: &str) -> Result<Self, DecodeError> {
        let bytes = BASE64_STANDARD.decode(base64)?;
        Ok(Self::new(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

impl HasMeta for Bytes {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Bytes {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_bare(writer, format!("{} bytes", self.value.len()))
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} bytes", self.value.len())
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

#[duplicate_item(
  _From;
  [Vec<u8>];
  [&[u8]];
)]
impl From<_From> for Bytes {
    fn from(bytes: _From) -> Self {
        Bytes::new(bytes)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.value
    }
}

impl<'own> From<&'own Bytes> for &'own [u8] {
    fn from(bytes: &'own Bytes) -> Self {
        &bytes.value
    }
}

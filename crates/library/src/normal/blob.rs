use super::super::meta::*;

use {
    base64::{prelude::*, *},
    bytes::*,
    bytestring::*,
    duplicate::*,
    kutil_cli::debug::*,
    std::{borrow::*, cmp::*, fmt, hash::*, io},
};

//
// Bytes
//

/// Normal bytes value.
///
/// Relies on [Bytes] for zero-copy cloning.
#[derive(Clone, Debug, Default, Eq)]
pub struct Blob {
    /// Actual value.
    pub value: Bytes,

    /// Metadata.
    pub meta: Meta,
}

impl Blob {
    /// Constructor.
    pub fn new(bytes: Bytes) -> Self {
        Self { value: bytes, ..Default::default() }
    }

    /// Constructor.
    pub fn new_from_base64<BytesT>(base64: BytesT) -> Result<Self, DecodeError>
    where
        BytesT: AsRef<[u8]>,
    {
        let bytes = BASE64_STANDARD.decode(base64)?;
        Ok(Self::from(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

impl HasMeta for Blob {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Blob {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, format!("{} bytes", self.value.len()))
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} bytes", self.value.len())
    }
}

// Delegated

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Blob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Blob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Blob {
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
  [Bytes];
  [Vec<u8>];
  [&'static [u8]];
)]
impl From<_From> for Blob {
    fn from(bytes: _From) -> Self {
        Blob::new(bytes.into())
    }
}

impl From<Cow<'_, [u8]>> for Blob {
    fn from(bytes: Cow<'_, [u8]>) -> Self {
        match bytes {
            Cow::Borrowed(bytes) => bytes.to_vec().into(),
            Cow::Owned(bytes) => bytes.into(),
        }
    }
}

#[duplicate_item(
  _From;
  [ByteString];
  [String];
  [&str];
)]
impl From<_From> for Blob {
    fn from(string: _From) -> Self {
        ByteString::from(string).into_bytes().into()
    }
}

impl From<Cow<'_, str>> for Blob {
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => string.into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl From<Blob> for Bytes {
    fn from(bytes: Blob) -> Self {
        bytes.value
    }
}

impl From<Blob> for Vec<u8> {
    fn from(bytes: Blob) -> Self {
        bytes.value.into()
    }
}

impl<'own> From<&'own Blob> for &'own [u8] {
    fn from(bytes: &'own Blob) -> Self {
        &bytes.value
    }
}

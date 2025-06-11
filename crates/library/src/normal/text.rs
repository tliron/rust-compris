use super::super::meta::*;

use {
    bytestring::*,
    duplicate::*,
    kutil_cli::debug::*,
    std::{borrow::*, cmp::*, fmt, hash::*, io},
};

//
// Text
//

/// Normal text value.
///
/// Relies on [ByteString] for zero-copy cloning.
///
/// We didn't call this struct "String" in order to avoid ambiguity with the built-in [String].
#[derive(Clone, Debug, Default, Eq)]
pub struct Text {
    /// Actual value.
    pub value: ByteString,

    /// Metadata.
    pub meta: Meta,
}

impl Text {
    /// Constructor.
    pub fn new(text: ByteString) -> Self {
        Self { value: text, ..Default::default() }
    }

    /// As string.
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

impl HasMeta for Text {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Text {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{}", context.theme.string(format!("{:?}", self.value)))
    }
}

impl fmt::Display for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

// Delegated

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Text {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

// Conversions

#[duplicate_item(
  _From;
  [ByteString];
  [String];
  [&str];
)]
impl From<_From> for Text {
    fn from(string: _From) -> Self {
        Self::new(string.into())
    }
}

impl From<Cow<'_, str>> for Text {
    fn from(string: Cow<'_, str>) -> Self {
        match string {
            Cow::Borrowed(string) => string.into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl From<Text> for String {
    fn from(text: Text) -> Self {
        text.into()
    }
}

impl<'own> From<&'own Text> for &'own str {
    fn from(text: &'own Text) -> Self {
        &text.value
    }
}

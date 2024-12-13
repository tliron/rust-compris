use super::super::meta::*;

use {
    duplicate::*,
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Text
//

/// Normal text value.
///
/// Why didn't we call this struct "String"? Honestly, just to avoid ambiguity with
/// the built-in [String]. But it's a string.
#[derive(Clone, Debug, Default, Eq)]
pub struct Text {
    /// Actual value.
    pub value: String,

    /// Metadata.
    pub meta: Meta,
}

impl Text {
    /// Constructor.
    pub fn new<StringT>(string: StringT) -> Self
    where
        StringT: Into<String>,
    {
        Self { value: string.into(), ..Default::default() }
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
  [String];
  [&str];
)]
impl From<_From> for Text {
    fn from(string: _From) -> Self {
        Text::new(string)
    }
}

impl From<Text> for String {
    fn from(text: Text) -> Self {
        text.value
    }
}

impl<'own> From<&'own Text> for &'own str {
    fn from(text: &'own Text) -> Self {
        &text.value
    }
}

use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Text
//

/// Normal text value.
///
/// Why didn't we call this struct "String"? Honestly, just to avoid ambiguity with
/// the built-in [String]. But it's a string.
#[derive(Debug, Default, Clone, Eq)]
pub struct Text {
    /// Actual value.
    pub value: String,

    /// Metadata.
    pub meta: Meta,
}

impl Text {
    /// Constructor.
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl Normal for Text {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        self.value.clone()
    }
}

impl Debuggable for Text {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let value = self.value.style(styles.string);
        write!(writer, "{:?}", value)?;
        if let Some(coordinates) = &self.meta.coordinates {
            write!(writer, " ")?;
            coordinates.write_debug_representation(writer, prefix, styles)?;
        }
        Ok(())
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

impl From<String> for Text {
    fn from(string: String) -> Self {
        Text::new(string)
    }
}

impl From<&str> for Text {
    fn from(string: &str) -> Self {
        Text::new(string)
    }
}

impl From<Text> for String {
    fn from(text: Text) -> Self {
        text.value
    }
}

impl<'a> From<&'a Text> for &'a str {
    fn from(text: &'a Text) -> Self {
        &text.value
    }
}

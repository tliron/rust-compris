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

impl WriteDebug for Text {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.string);
        write!(writer, "{:?}", value)?;
        if let Some(location) = &self.meta.location {
            write!(writer, " ")?;
            location.write_debug_representation(writer, indentation, styles)?;
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// Conversions

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::new(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text::new(value)
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.value
    }
}

impl<'a> From<&'a Text> for &'a str {
    fn from(value: &'a Text) -> Self {
        &value.value
    }
}

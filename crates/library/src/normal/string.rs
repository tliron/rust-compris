use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

//
// String
//

/// Normal string value.
#[derive(Debug, Default, Clone, Eq)]
pub struct String {
    /// Actual value.
    pub value: StdString,

    /// Metadata.
    pub meta: Meta,
}

impl String {
    /// Constructor.
    pub fn new(value: impl Into<StdString>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl From<StdString> for String {
    fn from(value: StdString) -> Self {
        String::new(value)
    }
}
impl From<String> for StdString {
    fn from(value: String) -> Self {
        value.value
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        String::new(value)
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Normal for String {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        self.value.clone()
    }
}

impl fmt::Display for String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.value)
    }
}

impl<W: io::Write> WriteDebug<W> for String {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = self.value.style(styles.string);
        write!(writer, "{:?}", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

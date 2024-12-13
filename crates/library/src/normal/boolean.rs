use super::super::*;

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Boolean
//

/// Normal boolean value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Boolean {
    /// Actual value.
    pub value: bool,

    /// Metadata.
    pub meta: Meta,
}

impl Boolean {
    /// Constructor.
    pub fn new(value: impl Into<bool>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl Normal for Boolean {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

impl WriteDebug for Boolean {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.plain);
        write!(writer, "{}", value)?;
        if let Some(location) = &self.meta.location {
            write!(writer, " ")?;
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

// Delegated

impl PartialEq for Boolean {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Boolean {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Boolean {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Boolean {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// Conversions

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean::new(value)
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.value
    }
}

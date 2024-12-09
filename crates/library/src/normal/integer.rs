use super::super::*;

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

//
// Integer
//

/// Normal integer value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Integer {
    /// Actual value.
    pub value: i64,

    /// Metadata.
    pub meta: Meta,
}

impl Integer {
    /// Constructor.
    pub fn new(value: impl Into<i64>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::new(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer::new(value as i64)
    }
}

impl From<i16> for Integer {
    fn from(value: i16) -> Self {
        Integer::new(value as i64)
    }
}

impl From<i8> for Integer {
    fn from(value: i8) -> Self {
        Integer::new(value as i64)
    }
}

impl From<Integer> for i64 {
    fn from(value: Integer) -> Self {
        value.value
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Normal for Integer {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        self.value.to_string()
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.value)
    }
}

impl<W: io::Write> WriteDebug<W> for Integer {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} i64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

//
// UnsignedInteger
//

/// Normal unsigned integer value.
#[derive(Debug, Default, Clone, Eq)]
pub struct UnsignedInteger {
    /// Actual value.
    pub value: u64,

    /// Metadata.
    pub meta: Meta,
}

impl UnsignedInteger {
    /// Constructor.
    pub fn new(value: impl Into<u64>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl From<u64> for UnsignedInteger {
    fn from(value: u64) -> Self {
        UnsignedInteger::new(value)
    }
}

impl From<u32> for UnsignedInteger {
    fn from(value: u32) -> Self {
        UnsignedInteger::new(value as u64)
    }
}

impl From<u16> for UnsignedInteger {
    fn from(value: u16) -> Self {
        UnsignedInteger::new(value as u64)
    }
}

impl From<u8> for UnsignedInteger {
    fn from(value: u8) -> Self {
        UnsignedInteger::new(value as u64)
    }
}

impl From<UnsignedInteger> for u64 {
    fn from(value: UnsignedInteger) -> Self {
        value.value
    }
}

impl PartialEq for UnsignedInteger {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for UnsignedInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for UnsignedInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for UnsignedInteger {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Normal for UnsignedInteger {
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

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u64", self.value)
    }
}

impl<W: io::Write> WriteDebug<W> for UnsignedInteger {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} u64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

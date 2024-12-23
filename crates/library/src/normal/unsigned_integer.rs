use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
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

impl Normal for UnsignedInteger {
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

impl Debuggable for UnsignedInteger {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let value = self.value.style(styles.number);
        write!(writer, "{} u64", value)?;
        if let Some(coordinates) = &self.meta.coordinates {
            write!(writer, " ")?;
            coordinates.write_debug_representation(writer, prefix, styles)?;
        }
        Ok(())
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u64", self.value)
    }
}

// Delegated

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
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

// Conversion

impl From<u64> for UnsignedInteger {
    fn from(unsigned_integer: u64) -> Self {
        UnsignedInteger::new(unsigned_integer)
    }
}

impl From<u32> for UnsignedInteger {
    fn from(unsigned_integer: u32) -> Self {
        UnsignedInteger::new(unsigned_integer as u64)
    }
}

impl From<u16> for UnsignedInteger {
    fn from(unsigned_integer: u16) -> Self {
        UnsignedInteger::new(unsigned_integer as u64)
    }
}

impl From<u8> for UnsignedInteger {
    fn from(unsigned_integer: u8) -> Self {
        UnsignedInteger::new(unsigned_integer as u64)
    }
}

impl From<UnsignedInteger> for u64 {
    fn from(unsigned_integer: UnsignedInteger) -> Self {
        unsigned_integer.value
    }
}

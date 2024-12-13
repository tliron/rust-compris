use super::super::meta::*;

use {
    duplicate::*,
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
    pub fn new<UnsignedIntegerT>(unsigned_integer: UnsignedIntegerT) -> Self
    where
        UnsignedIntegerT: Into<u64>,
    {
        Self { value: unsigned_integer.into(), ..Default::default() }
    }
}

impl HasMeta for UnsignedInteger {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for UnsignedInteger {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} u64", self.value.style(theme.number))
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

#[duplicate_item(
    _From;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl From<_From> for UnsignedInteger {
    fn from(unsigned_integer: _From) -> Self {
        UnsignedInteger::new(unsigned_integer as u64)
    }
}

impl From<&UnsignedInteger> for u64 {
    fn from(unsigned_integer: &UnsignedInteger) -> Self {
        unsigned_integer.value
    }
}

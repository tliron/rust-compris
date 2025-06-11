use super::super::meta::*;

use {
    duplicate::*,
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Integer
//

/// Normal integer value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Integer {
    /// Actual value.
    pub value: i64,

    /// Metadata.
    pub meta: Meta,
}

impl Integer {
    /// Constructor.
    pub fn new(integer: i64) -> Self {
        Self { value: integer, ..Default::default() }
    }
}

impl HasMeta for Integer {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Integer {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if matches!(context.format, DebugFormat::Compact) {
            context.theme.write_number(writer, self.value)
        } else {
            write!(writer, "{} {}", context.theme.number(self.value), context.theme.meta("i64"))
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.value)
    }
}

// Delegated

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
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl From<_From> for Integer {
    fn from(integer: _From) -> Self {
        Self::new(integer as i64)
    }
}

impl From<&Integer> for i64 {
    fn from(integer: &Integer) -> Self {
        integer.value
    }
}

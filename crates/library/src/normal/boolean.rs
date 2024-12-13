use super::super::meta::*;

use {
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Boolean
//

/// Normal boolean value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Boolean {
    /// Actual value.
    pub value: bool,

    /// Metadata.
    pub meta: Meta,
}

impl Boolean {
    /// Constructor.
    pub fn new<BooleanT>(boolean: BooleanT) -> Self
    where
        BooleanT: Into<bool>,
    {
        Self { value: boolean.into(), ..Default::default() }
    }
}

impl HasMeta for Boolean {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Boolean {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_bare(writer, self.value)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
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
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

// Conversions

impl From<bool> for Boolean {
    fn from(boolean: bool) -> Self {
        Boolean::new(boolean)
    }
}

impl From<&Boolean> for bool {
    fn from(boolean: &Boolean) -> Self {
        boolean.value
    }
}

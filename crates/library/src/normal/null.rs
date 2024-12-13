use super::super::meta::*;

use {
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Null
//

/// Normal null value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Null {
    /// Metadata.
    pub meta: Meta,
}

impl Null {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasMeta for Null {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Null {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_bare(writer, "Null")
    }
}

impl fmt::Display for Null {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("Null", formatter)
    }
}

// Basics

impl PartialEq for Null {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialOrd for Null {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for Null {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Hash for Null {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        ().hash(state)
    }
}

// Conversions

impl From<()> for Null {
    fn from(_: ()) -> Self {
        Self::new()
    }
}

impl From<Null> for () {
    fn from(_: Null) -> Self {
        ()
    }
}

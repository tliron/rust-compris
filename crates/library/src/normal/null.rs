use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Null
//

/// Normal null value.
#[derive(Debug, Default, Clone, Eq)]
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

impl Normal for Null {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        "null".into()
    }
}

impl Debuggable for Null {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let value = "null".style(styles.plain);
        write!(writer, "{}", value)?;
        if let Some(coordinates) = &self.meta.coordinates {
            write!(writer, " ")?;
            coordinates.write_debug_representation(writer, prefix, styles)?;
        }
        Ok(())
    }
}

impl fmt::Display for Null {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        "null".fmt(formatter)
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
        state.write_u8(0)
    }
}

// Conversions

impl From<()> for Null {
    fn from(_: ()) -> Self {
        Self::new()
    }
}

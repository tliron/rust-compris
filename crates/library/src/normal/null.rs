use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
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

impl From<()> for Null {
    fn from(_: ()) -> Self {
        Self::new()
    }
}

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
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(0)
    }
}

impl Normal for Null {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        "null".into()
    }
}

impl fmt::Display for Null {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        "null".fmt(formatter)
    }
}

impl<W: io::Write> WriteDebug<W> for Null {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = "null".style(styles.plain);
        write!(writer, "{}", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

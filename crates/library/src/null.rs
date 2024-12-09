use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    owo_colors::OwoColorize,
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

impl Into<Value> for Null {
    fn into(self) -> Value {
        Value::Null(self)
    }
}

impl HasMeta for Null {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
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

impl fmt::Display for Null {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        "null".fmt(formatter)
    }
}

impl<W: io::Write> WriteDebug<W> for Null {
    fn write_debug_representation(
        &self,
        writer: &mut W,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = "null".style(styles.plain);
        write!(writer, "{}", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Null {
    fn to_map_string_key(&self) -> String {
        "null".into()
    }
}

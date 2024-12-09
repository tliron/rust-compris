use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Boolean
//

/// Normal boolean value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Boolean {
    /// Actual value.
    pub value: bool,

    /// Metadata.
    pub meta: Meta,
}

impl Boolean {
    /// Constructor.
    pub fn new(value: bool) -> Self {
        Self { value, ..Default::default() }
    }
}

impl Into<Value> for Boolean {
    fn into(self) -> Value {
        Value::Boolean(self)
    }
}

impl HasMeta for Boolean {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl<W: io::Write> WriteDebug<W> for Boolean {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = self.value.style(styles.plain);
        write!(writer, "{}", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Boolean {
    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

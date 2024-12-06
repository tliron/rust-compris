use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string},
};

//
// String
//

/// ARD string value.
#[derive(Debug, Default, Clone, Eq)]
pub struct String {
    pub value: string::String,
    pub meta: Meta,
}

impl String {
    pub fn new(value: string::String) -> Self {
        Self { value, ..Default::default() }
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}

impl HasMeta for String {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.value)
    }
}

impl WriteDebug for String {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.string);
        write!(writer, "{:?}", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, _indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for String {
    fn to_map_string_key(&self) -> string::String {
        self.value.clone()
    }
}

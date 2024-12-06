use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    base64::prelude::*,
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Bytes
//

/// ARD bytes value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Bytes {
    pub value: Vec<u8>,
    pub meta: Meta,
}

impl Bytes {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value, ..Default::default() }
    }
}

impl Into<Value> for Bytes {
    fn into(self) -> Value {
        Value::Bytes(self)
    }
}

impl HasMeta for Bytes {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Bytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        BASE64_STANDARD.encode(&self.value).fmt(formatter)
    }
}

impl WriteDebug for Bytes {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        write!(writer, "{} bytes", self.value.len().style(styles.plain))?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, _indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Bytes {
    fn to_map_string_key(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

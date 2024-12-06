use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// UnsignedInteger
//

/// ARD unsigned integer value.
#[derive(Debug, Default, Clone, Eq)]
pub struct UnsignedInteger {
    pub value: u64,
    pub meta: Meta,
}

impl UnsignedInteger {
    pub fn new(value: u64) -> Self {
        Self { value, ..Default::default() }
    }
}

impl Into<Value> for UnsignedInteger {
    fn into(self) -> Value {
        Value::UnsignedInteger(self)
    }
}

impl HasMeta for UnsignedInteger {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u64", self.value)
    }
}

impl WriteDebug for UnsignedInteger {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} u64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, _indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for UnsignedInteger {
    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

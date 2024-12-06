use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Integer
//

/// ARD integer value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Integer {
    pub value: i64,
    pub meta: Meta,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self { value, ..Default::default() }
    }
}

impl Into<Value> for Integer {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}

impl HasMeta for Integer {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.value)
    }
}

impl WriteDebug for Integer {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} i64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, _indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Integer {
    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    ordered_float::*,
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Float
//

/// ARD floating point value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Float {
    pub value: OrderedFloat<f64>,
    pub meta: Meta,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Self { value: OrderedFloat(value), ..Default::default() }
    }
}

impl Into<Value> for Float {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl HasMeta for Float {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Float {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}f64", self.value)
    }
}

impl WriteDebug for Float {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} f64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, _indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Float {
    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

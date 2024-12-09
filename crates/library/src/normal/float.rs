use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use {
    ordered_float::*,
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

//
// Float
//

/// Normal floating point value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Float {
    /// Actual value.
    pub value: OrderedFloat<f64>,

    /// Metadata.
    pub meta: Meta,
}

impl Float {
    /// Constructor.
    pub fn new(value: impl Into<f64>) -> Self {
        Self { value: OrderedFloat(value.into()), ..Default::default() }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::new(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::new(value as f64)
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.value.into()
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

impl Normal for Float {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        self.value.to_string()
    }
}

impl fmt::Display for Float {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}f64", self.value)
    }
}

impl<W: io::Write> WriteDebug<W> for Float {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        let value = self.value.style(styles.number);
        write!(writer, "{} f64", value)?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

use super::{super::normal::*, meta::*};

use {
    kutil_cli::debug::*,
    ordered_float::*,
    owo_colors::*,
    std::{cmp::*, fmt, hash::*, io},
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

impl Normal for Float {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> String {
        self.value.to_string()
    }
}

impl Debuggable for Float {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let value = self.value.style(styles.number);
        write!(writer, "{} f64", value)?;
        if let Some(coordinates) = &self.meta.coordinates {
            write!(writer, " ")?;
            coordinates.write_debug_representation(writer, prefix, styles)?;
        }
        Ok(())
    }
}

impl fmt::Display for Float {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}f64", self.value)
    }
}

// Delegated

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
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

// Conversions

impl From<f64> for Float {
    fn from(float: f64) -> Self {
        Float::new(float)
    }
}

impl From<f32> for Float {
    fn from(float: f32) -> Self {
        Float::new(float as f64)
    }
}

impl From<Float> for f64 {
    fn from(float: Float) -> Self {
        float.value.into()
    }
}

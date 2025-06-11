use super::super::meta::*;

use {
    duplicate::*,
    kutil_cli::debug::*,
    ordered_float::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Float
//

/// Normal floating point value.
#[derive(Clone, Debug, Default, Eq)]
pub struct Float {
    /// Actual value.
    pub value: OrderedFloat<f64>,

    /// Metadata.
    pub meta: Meta,
}

impl Float {
    /// Constructor.
    pub fn new(float: OrderedFloat<f64>) -> Self {
        Self { value: float, ..Default::default() }
    }

    /// Constructor.
    pub fn new_from(float: OrderedFloat<f64>) -> Self {
        Self { value: float, ..Default::default() }
    }
}

impl HasMeta for Float {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }
}

impl Debuggable for Float {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if matches!(context.format, DebugFormat::Compact) {
            context.theme.write_number(writer, self.value)
        } else {
            write!(writer, "{} {}", context.theme.number(self.value), context.theme.meta("f64"))
        }
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

#[duplicate_item(
  _From;
  [f64];
  [f32];
)]
impl From<_From> for Float {
    fn from(float: _From) -> Self {
        Self::new((float as f64).into())
    }
}

impl From<OrderedFloat<f64>> for Float {
    fn from(float: OrderedFloat<f64>) -> Self {
        Self::new(float)
    }
}

impl From<Float> for f64 {
    fn from(float: Float) -> Self {
        float.value.into()
    }
}

impl From<Float> for OrderedFloat<f64> {
    fn from(float: Float) -> Self {
        float.value
    }
}

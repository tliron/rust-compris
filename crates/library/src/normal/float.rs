use {
    super::super::annotation::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    duplicate::*,
    kutil_cli::debug::*,
    ordered_float::*,
    std::{fmt, io},
};

//
// Float
//

impl_normal! {
    /// Normal floating point value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is a [OrderedFloat] in order to support comparison and hashing.
    Float(OrderedFloat<f64>)
}

impl_normal_basic!(Float);

impl<AnnotationsT> Debuggable for Float<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
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

impl<AnnotationsT> fmt::Display for Float<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}f64", self.value)
    }
}

// Conversions

#[duplicate_item(
  ToNormalT;
  [f64];
  [f32];
)]
impl<AnnotationsT> From<ToNormalT> for Float<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(float: ToNormalT) -> Self {
        Self::new((float as f64).into())
    }
}

impl<AnnotationsT> From<OrderedFloat<f64>> for Float<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(float: OrderedFloat<f64>) -> Self {
        Self::new(float)
    }
}

impl<AnnotationsT> From<Float<AnnotationsT>> for f64 {
    fn from(float: Float<AnnotationsT>) -> Self {
        float.value.into()
    }
}

impl<AnnotationsT> From<Float<AnnotationsT>> for OrderedFloat<f64> {
    fn from(float: Float<AnnotationsT>) -> Self {
        float.value
    }
}

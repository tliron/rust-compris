use {
    super::super::annotate::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    duplicate::*,
    kutil::cli::depict::*,
    ordered_float::*,
    std::{fmt, io},
};

//
// Float
//

impl_normal! {
    /// Normal floating point variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    ///
    /// Note that the value is an [OrderedFloat] in order to support comparison and hashing.
    Float(OrderedFloat<f64>)
}

impl_normal_basic!(Float);

impl<AnnotatedT> Depict for Float<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        if context.get_format() == DepictionFormat::Compact {
            context.theme.write_number(writer, self.inner)
        } else {
            write!(writer, "{} {}", context.theme.number(self.inner), context.theme.meta("f64"))
        }
    }
}

impl<AnnotatedT> fmt::Display for Float<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}f64", self.inner)
    }
}

// Conversions

#[duplicate_item(
  ToNormalT;
  [f64];
  [f32];
)]
impl<AnnotatedT> From<ToNormalT> for Float<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(float: ToNormalT) -> Self {
        let float: OrderedFloat<_> = (float as f64).into();
        Self::from(float)
    }
}

impl<AnnotatedT> From<Float<AnnotatedT>> for f64 {
    fn from(float: Float<AnnotatedT>) -> Self {
        float.inner.into()
    }
}

impl<AnnotatedT> From<&Float<AnnotatedT>> for f64 {
    fn from(float: &Float<AnnotatedT>) -> Self {
        float.inner.into()
    }
}

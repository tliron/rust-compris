use {
    super::super::annotate::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    duplicate::*,
    kutil::cli::debug::*,
    std::{fmt, io},
};

//
// Integer
//

impl_normal! {
    /// Normal integer variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    Integer(i64)
}

impl_normal_basic!(Integer);

impl<AnnotatedT> Debuggable for Integer<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if context.format == DebugFormat::Compact {
            context.theme.write_number(writer, self.inner)
        } else {
            write!(writer, "{} {}", context.theme.number(self.inner), context.theme.meta("i64"))
        }
    }
}

impl<AnnotatedT> fmt::Display for Integer<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.inner)
    }
}

// Conversions

#[duplicate_item(
  ToNormalT;
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl<AnnotatedT> From<ToNormalT> for Integer<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(integer: ToNormalT) -> Self {
        Self::new(integer as i64)
    }
}

impl<AnnotatedT> From<&Integer<AnnotatedT>> for i64 {
    fn from(integer: &Integer<AnnotatedT>) -> Self {
        integer.inner
    }
}

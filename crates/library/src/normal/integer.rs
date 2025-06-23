use {
    super::super::annotation::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    duplicate::*,
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// Integer
//

impl_normal! {
    /// Normal integer value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    Integer(i64)
}

impl_normal_basic!(Integer);

impl<AnnotationsT> Debuggable for Integer<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if matches!(context.format, DebugFormat::Compact) {
            context.theme.write_number(writer, self.value)
        } else {
            write!(writer, "{} {}", context.theme.number(self.value), context.theme.meta("i64"))
        }
    }
}

impl<AnnotationsT> fmt::Display for Integer<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}i64", self.value)
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
impl<AnnotationsT> From<ToNormalT> for Integer<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(integer: ToNormalT) -> Self {
        Self::new(integer as i64)
    }
}

impl<AnnotationsT> From<&Integer<AnnotationsT>> for i64 {
    fn from(integer: &Integer<AnnotationsT>) -> Self {
        integer.value
    }
}

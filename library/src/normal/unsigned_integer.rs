use {
    super::super::annotate::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    duplicate::*,
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// UnsignedInteger
//

impl_normal! {
    /// Normal unsigned integer variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    UnsignedInteger(u64)
}

impl_normal_basic!(UnsignedInteger);

impl<AnnotatedT> Debuggable for UnsignedInteger<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if context.format == DebugFormat::Compact {
            context.theme.write_number(writer, self.inner)
        } else {
            write!(writer, "{} {}", context.theme.number(self.inner), context.theme.meta("u64"))
        }
    }
}

impl<AnnotatedT> fmt::Display for UnsignedInteger<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u64", self.inner)
    }
}

// Conversion

#[duplicate_item(
  ToNormalT;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl<AnnotatedT> From<ToNormalT> for UnsignedInteger<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(unsigned_integer: ToNormalT) -> Self {
        Self::new(unsigned_integer as u64)
    }
}

impl<AnnotatedT> From<&UnsignedInteger<AnnotatedT>> for u64 {
    fn from(unsigned_integer: &UnsignedInteger<AnnotatedT>) -> Self {
        unsigned_integer.inner
    }
}

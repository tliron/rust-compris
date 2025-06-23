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
// UnsignedInteger
//

impl_normal! {
    /// Normal unsigned integer value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    UnsignedInteger(u64)
}

impl_normal_basic!(UnsignedInteger);

impl<AnnotationsT> Debuggable for UnsignedInteger<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if matches!(context.format, DebugFormat::Compact) {
            context.theme.write_number(writer, self.value)
        } else {
            write!(writer, "{} {}", context.theme.number(self.value), context.theme.meta("u64"))
        }
    }
}

impl<AnnotationsT> fmt::Display for UnsignedInteger<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u64", self.value)
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
impl<AnnotationsT> From<ToNormalT> for UnsignedInteger<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(unsigned_integer: ToNormalT) -> Self {
        Self::new(unsigned_integer as u64)
    }
}

impl<AnnotationsT> From<&UnsignedInteger<AnnotationsT>> for u64 {
    fn from(unsigned_integer: &UnsignedInteger<AnnotationsT>) -> Self {
        unsigned_integer.value
    }
}

use {
    super::super::annotation::*,
    crate::{impl_normal, impl_normal_basic},
};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// Boolean
//

impl_normal! {
    /// Normal boolean value.
    ///
    /// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
    Boolean(bool)
}

impl_normal_basic!(Boolean);

impl<AnnotationsT> Debuggable for Boolean<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, self.value)
    }
}

impl<AnnotationsT> fmt::Display for Boolean<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, formatter)
    }
}

// Conversions

impl<AnnotationsT> From<bool> for Boolean<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(boolean: bool) -> Self {
        Self::new(boolean)
    }
}

impl<AnnotationsT> From<&Boolean<AnnotationsT>> for bool {
    fn from(boolean: &Boolean<AnnotationsT>) -> Self {
        boolean.value
    }
}

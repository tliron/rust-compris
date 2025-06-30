use {
    super::super::annotate::*,
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
    /// Normal boolean variant.
    ///
    /// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
    Boolean(bool)
}

impl_normal_basic!(Boolean);

impl<AnnotatedT> Debuggable for Boolean<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, self.inner)
    }
}

impl<AnnotatedT> fmt::Display for Boolean<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

// Conversions

impl<AnnotatedT> From<bool> for Boolean<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(boolean: bool) -> Self {
        Self::new(boolean)
    }
}

impl<AnnotatedT> From<&Boolean<AnnotatedT>> for bool {
    fn from(boolean: &Boolean<AnnotatedT>) -> Self {
        boolean.inner
    }
}

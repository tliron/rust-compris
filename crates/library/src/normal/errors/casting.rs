use {super::super::value::*, crate::impl_annotated};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// CastingError
//

/// Casting error.
#[derive(Debug, Error)]
pub struct CastingError<AnnotatedT> {
    /// Value.
    pub value: Value<AnnotatedT>,

    /// Type name.
    pub type_name: String,
}

impl<AnnotatedT> CastingError<AnnotatedT> {
    /// Constructor.
    pub fn new(value: &Value<AnnotatedT>, type_name: String) -> Self
    where
        AnnotatedT: Clone + Default,
    {
        Self { value: value.clone(), type_name }
    }
}

impl_annotated!(CastingError, value);

impl<AnnotatedT> Debuggable for CastingError<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} cannot be cast to a {}", self.value, context.theme.error(&self.type_name))
    }
}

impl<AnnotatedT> fmt::Display for CastingError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.value, self.type_name)
    }
}

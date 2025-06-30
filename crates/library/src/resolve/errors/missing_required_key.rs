use {super::super::super::normal::*, crate::impl_annotated};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MissingRequiredKeyError
//

/// Missing required key.
#[derive(Debug, Error)]
pub struct MissingRequiredKeyError<AnnotatedT> {
    /// Key.
    pub key: Value<AnnotatedT>,
}

impl<AnnotatedT> MissingRequiredKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new<KeyT>(key: KeyT) -> Self
    where
        KeyT: Into<Value<AnnotatedT>>,
    {
        Self { key: key.into() }
    }
}

impl_annotated!(MissingRequiredKeyError, key);

impl<AnnotatedT> Debuggable for MissingRequiredKeyError<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "missing required key: {}", context.theme.error(key))
    }
}

impl<AnnotatedT> fmt::Display for MissingRequiredKeyError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}

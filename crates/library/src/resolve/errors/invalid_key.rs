use {super::super::super::normal::*, crate::impl_annotated};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// InvalidKeyError
//

/// Invalid key.
#[derive(Debug, Error)]
pub struct InvalidKeyError<AnnotatedT> {
    /// Key.
    pub key: Value<AnnotatedT>,
}

impl<AnnotatedT> InvalidKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new<KeyT>(key: KeyT) -> Self
    where
        KeyT: Into<Value<AnnotatedT>>,
    {
        Self { key: key.into() }
    }
}

impl_annotated!(InvalidKeyError, key);

impl<AnnotatedT> Debuggable for InvalidKeyError<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "invalid key: {}", context.theme.error(key))
    }
}

impl<AnnotatedT> fmt::Display for InvalidKeyError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}

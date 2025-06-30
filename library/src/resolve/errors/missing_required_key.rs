use {
    super::super::super::{annotate::*, normal::*},
    crate::impl_annotated,
};

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
    pub key: Variant<AnnotatedT>,
}

impl<AnnotatedT> MissingRequiredKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new(key: Variant<AnnotatedT>) -> Self {
        Self { key }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> MissingRequiredKeyError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        MissingRequiredKeyError { key: self.key.into_annotated() }
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

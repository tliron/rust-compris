use {super::super::super::annotate::*, crate::impl_annotated};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MalformedError
//

/// Malformed error.
#[derive(Debug, Error)]
pub struct MalformedError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> MalformedError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, reason: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, reason, annotated: Default::default() }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> MalformedError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        MalformedError { type_name: self.type_name, reason: self.reason, annotated: Default::default() }
            .with_annotations_from(&self.annotated)
    }
}

impl_annotated!(MalformedError);

impl<AnnotatedT> Debuggable for MalformedError<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "malformed: {}: {}", self.type_name, context.theme.error(&self.reason))
    }
}

impl<AnnotatedT> fmt::Display for MalformedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}

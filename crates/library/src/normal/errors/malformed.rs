use super::super::super::annotation::*;

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
pub struct MalformedError<AnnotationsT> {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,

    /// Annotations.
    pub annotations: AnnotationsT,
}

impl<AnnotationsT> MalformedError<AnnotationsT> {
    /// Constructor.
    pub fn new(type_name: &str, reason: &str) -> Self
    where
        AnnotationsT: Default,
    {
        Self { type_name: type_name.into(), reason: reason.into(), annotations: AnnotationsT::default() }
    }
}

impl<AnnotationsT> Annotated for MalformedError<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        self.annotations.get_annotations()
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotations.get_annotations_mut()
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        self.annotations.set_annotations(annotations);
    }
}

impl<AnnotationsT> Debuggable for MalformedError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "malformed: {}: {}", self.type_name, context.theme.error(&self.reason))
    }
}

impl<AnnotationsT> fmt::Display for MalformedError<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}

use super::super::super::annotation::*;

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
pub struct CastingError<AnnotationsT> {
    /// Value.
    pub value: String,

    /// Type name.
    pub type_name: String,

    /// Annotations.
    pub annotations: AnnotationsT,
}

impl<AnnotationsT> CastingError<AnnotationsT> {
    /// Constructor.
    pub fn new(value: &str, type_name: &str) -> Self
    where
        AnnotationsT: Default,
    {
        Self { value: value.into(), type_name: type_name.into(), annotations: AnnotationsT::default() }
    }
}

impl<AnnotationsT> Annotated for CastingError<AnnotationsT>
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

impl<AnnotationsT> Debuggable for CastingError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} cannot be cast to a {}", self.value, context.theme.error(&self.type_name))
    }
}

impl<AnnotationsT> fmt::Display for CastingError<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.value, self.type_name)
    }
}

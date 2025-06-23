use super::super::super::annotation::*;

use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// GrammarError
//

/// Grammar error.
#[derive(Debug, Error)]
pub struct GrammarError<AnnotationsT> {
    /// Message.
    pub message: String,

    /// Annotations.
    pub annotations: Option<AnnotationsT>,
}

impl<AnnotationsT> GrammarError<AnnotationsT> {
    /// Constructor.
    pub fn new(message: String, annotations: Option<AnnotationsT>) -> Self {
        Self { message, annotations }
    }
}

impl<AnnotationsT> Annotated for GrammarError<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        self.annotations.as_ref().and_then(|annotations| annotations.get_annotations())
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotations.as_mut().and_then(|annotations| annotations.get_annotations_mut())
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        if let Some(self_annotations) = self.annotations.as_mut() {
            self_annotations.set_annotations(annotations);
        }
    }
}

impl<AnnotationsT> Debuggable for GrammarError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_error(writer, &self.message)
    }
}

impl<AnnotationsT> From<String> for GrammarError<AnnotationsT> {
    fn from(message: String) -> Self {
        Self { message, annotations: None }
    }
}

impl<AnnotationsT> From<&str> for GrammarError<AnnotationsT> {
    fn from(message: &str) -> Self {
        message.to_string().into()
    }
}

impl<AnnotationsT> fmt::Display for GrammarError<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

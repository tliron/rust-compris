use super::super::super::{annotation::*, normal::*};

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
pub struct InvalidKeyError<AnnotationsT> {
    /// Key.
    pub key: Value<AnnotationsT>,
}

impl<AnnotationsT> InvalidKeyError<AnnotationsT> {
    /// Constructor.
    pub fn new<KeyT>(key: KeyT) -> Self
    where
        KeyT: Into<Value<AnnotationsT>>,
    {
        Self { key: key.into() }
    }
}

impl<AnnotationsT> Annotated for InvalidKeyError<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        self.key.get_annotations()
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.key.get_annotations_mut()
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        self.key.set_annotations(annotations);
    }
}

impl<AnnotationsT> Debuggable for InvalidKeyError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "invalid key: {}", context.theme.error(key))
    }
}

impl<AnnotationsT> fmt::Display for InvalidKeyError<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.key, formatter)
    }
}

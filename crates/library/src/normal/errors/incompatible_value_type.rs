use super::super::{super::annotation::*, value::*};

use {
    kutil_cli::debug::*,
    kutil_std::string::*,
    std::{fmt, io},
    thiserror::*,
};

//
// IncompatibleValueTypeError
//

/// Incompatible value type.
#[derive(Debug, Error)]
pub struct IncompatibleValueTypeError<AnnotationsT> {
    /// Expected type names.
    pub expected_type_names: Vec<String>,

    /// Type name.
    pub type_name: String,

    /// Annotations.
    pub annotations: AnnotationsT,
}

impl<AnnotationsT> IncompatibleValueTypeError<AnnotationsT> {
    /// Constructor.
    pub fn new(value: &Value<AnnotationsT>, expected_type_names: &[&str]) -> Self
    where
        AnnotationsT: Annotated + Default + Clone,
    {
        Self {
            expected_type_names: expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
            type_name: value.get_type_name().into(),
            annotations: AnnotationsT::default(),
        }
        .with_annotations_from(value)
    }
}

impl<AnnotationsT> Annotated for IncompatibleValueTypeError<AnnotationsT>
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

impl<AnnotationsT> Debuggable for IncompatibleValueTypeError<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(
            writer,
            "incompatible value type: is {}, expected {}",
            context.theme.error(&self.type_name),
            self.expected_type_names.join_conjunction("or")
        )
    }
}

impl<AnnotationsT> fmt::Display for IncompatibleValueTypeError<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}

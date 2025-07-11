use {
    super::super::{super::annotate::*, variant::*},
    crate::impl_annotated,
};

use {
    kutil_cli::debug::*,
    kutil_std::string::*,
    std::{fmt, io},
    thiserror::*,
};

//
// IncompatibleVariantTypeError
//

/// Incompatible value type.
#[derive(Debug, Error)]
pub struct IncompatibleVariantTypeError<AnnotatedT> {
    /// Expected type names.
    pub expected_type_names: Vec<String>,

    /// Type name.
    pub type_name: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<AnnotatedT> IncompatibleVariantTypeError<AnnotatedT> {
    /// Constructor.
    pub fn new(value: &Variant<AnnotatedT>, expected_type_names: &[&str]) -> Self
    where
        AnnotatedT: Annotated + Default + Clone,
    {
        Self {
            expected_type_names: expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
            type_name: value.get_type_name().into(),
            annotated: AnnotatedT::default(),
        }
        .with_annotations_from(value)
    }
}

impl_annotated!(IncompatibleVariantTypeError);

impl<AnnotatedT> Debuggable for IncompatibleVariantTypeError<AnnotatedT> {
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

impl<AnnotatedT> fmt::Display for IncompatibleVariantTypeError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}

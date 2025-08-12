use {
    super::super::{super::annotate::*, variant::*},
    crate::impl_annotated,
};

use {
    kutil::cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// CastingError
//

/// Casting error.
#[derive(Debug, Error)]
pub struct CastingError<AnnotatedT> {
    /// Variant.
    pub variant: Variant<AnnotatedT>,

    /// Type name.
    pub type_name: String,
}

impl<AnnotatedT> CastingError<AnnotatedT> {
    /// Constructor.
    pub fn new(variant: Variant<AnnotatedT>, type_name: String) -> Self {
        Self { variant, type_name }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> CastingError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        CastingError { variant: self.variant.into_annotated(), type_name: self.type_name }
    }
}

impl_annotated!(CastingError, variant);

impl<AnnotatedT> Debuggable for CastingError<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} cannot be cast to a {}", self.variant, context.theme.error(&self.type_name))
    }
}

impl<AnnotatedT> fmt::Display for CastingError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.variant, self.type_name)
    }
}

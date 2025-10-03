use super::super::super::{annotate::*, normal::*};

use {
    kutil::cli::depict::*,
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
    pub key: Variant<AnnotatedT>,
}

impl<AnnotatedT> InvalidKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new(key: Variant<AnnotatedT>) -> Self {
        Self { key }
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> InvalidKeyError<NewAnnotationsT>
    where
        AnnotatedT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        InvalidKeyError::new(self.key.into_annotated())
    }
}

impl_annotated!(InvalidKeyError, key);

impl<AnnotatedT> Depict for InvalidKeyError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
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

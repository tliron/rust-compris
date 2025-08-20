use super::super::{super::annotate::*, variant::*};

use {kutil::cli::depict::*, std::io};

//
// AnnotatedDepictVariant
//

/// [Depict] wrapper for a [Variant] with [Annotations].
pub struct AnnotatedDepictVariant<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own Variant<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'own, AnnotatedT> AnnotatedDepictVariant<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own Variant<AnnotatedT>, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Depict for AnnotatedDepictVariant<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.inner {
            Variant::List(list) => list.annotated_depict(self.mode).depict(writer, context),
            Variant::Map(map) => map.annotated_depict(self.mode).depict(writer, context),
            _ => AnnotatedDepiction::new(self.inner, self.mode).depict(writer, context),
        }
    }
}

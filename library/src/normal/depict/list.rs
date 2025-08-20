use super::{
    super::{super::annotate::*, list::*},
    variant::*,
};

use {kutil::cli::depict::*, std::io};

//
// AnnotatedDepictList
//

/// [Depict] wrapper for a [List] with [Annotations].
pub struct AnnotatedDepictList<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own List<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'own, AnnotatedT> AnnotatedDepictList<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own List<AnnotatedT>, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Depict for AnnotatedDepictList<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> =
            self.inner.into_iter().map(|value| AnnotatedDepictVariant::new(value, self.mode)).collect();
        utils::depict_list(vector.iter(), None, writer, context)
    }
}

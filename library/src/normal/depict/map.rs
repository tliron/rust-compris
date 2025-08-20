use super::{
    super::{super::annotate::*, map::*},
    variant::*,
};

use {kutil::cli::depict::*, std::io};

//
// AnnotatedDepictMap
//

/// [Depict] wrapper for a [Map] with [Annotations].
pub struct AnnotatedDepictMap<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own Map<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'own, AnnotatedT> AnnotatedDepictMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own Map<AnnotatedT>, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Depict for AnnotatedDepictMap<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> = self
            .inner
            .into_iter()
            .map(|(key, value)| {
                (AnnotatedDepictVariant::new(key, self.mode), AnnotatedDepictVariant::new(value, self.mode))
            })
            .collect();
        let vector: Vec<_> = vector.iter().map(|(k, v)| (k, v)).collect();
        utils::depict_map(vector.into_iter(), None, writer, context)?;
        Ok(())
    }
}

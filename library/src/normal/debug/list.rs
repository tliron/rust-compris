use super::{
    super::{super::annotate::*, list::*},
    variant::*,
};

use {kutil::cli::debug::*, std::io};

//
// AnnotatedDebuggableList
//

/// [Debuggable] wrapper for a [List] with [Annotations].
pub struct AnnotatedDebuggableList<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own List<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableList<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own List<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Debuggable for AnnotatedDebuggableList<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> =
            self.inner.into_iter().map(|value| AnnotatedDebuggableVariant::new(value, self.mode)).collect();
        utils::write_debug_as_list(vector.iter(), None, writer, context)
    }
}

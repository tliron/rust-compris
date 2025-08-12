use super::{
    super::{super::annotate::*, map::*},
    variant::*,
};

use {kutil::cli::debug::*, std::io};

//
// AnnotatedDebuggableMap
//

/// [Debuggable] wrapper for a [Map] with [Annotations].
pub struct AnnotatedDebuggableMap<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own Map<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own Map<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Debuggable for AnnotatedDebuggableMap<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> = self
            .inner
            .into_iter()
            .map(|(key, value)| {
                (AnnotatedDebuggableVariant::new(key, self.mode), AnnotatedDebuggableVariant::new(value, self.mode))
            })
            .collect();
        let vector: Vec<_> = vector.iter().map(|(k, v)| (k, v)).collect();
        utils::write_debug_as_map(vector.into_iter(), None, writer, context)?;
        Ok(())
    }
}

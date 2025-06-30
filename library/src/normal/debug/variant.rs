use super::super::{super::annotate::*, variant::*};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableVariant
//

/// [Debuggable] wrapper for a [Variant] with [Annotations].
pub struct AnnotatedDebuggableVariant<'own, AnnotatedT> {
    /// Inner.
    pub inner: &'own Variant<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableVariant<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'own Variant<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, AnnotatedT> Debuggable for AnnotatedDebuggableVariant<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.inner {
            Variant::List(list) => list.annotated_debuggable(self.mode).write_debug_for(writer, context),
            Variant::Map(map) => map.annotated_debuggable(self.mode).write_debug_for(writer, context),
            _ => AnnotatedDebuggable::new(self.inner, self.mode).write_debug_for(writer, context),
        }
    }
}

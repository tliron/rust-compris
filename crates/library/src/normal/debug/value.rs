use super::super::{super::annotation::*, value::*};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableValue
//

///
pub struct AnnotatedDebuggableValue<'own, AnnotatedT> {
    /// Value.
    pub value: &'own Value<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableValue<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { value, mode }
    }
}

impl<'own, AnnotatedT> Debuggable for AnnotatedDebuggableValue<'own, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.value {
            Value::List(list) => list.annotated_debuggable(self.mode).write_debug_for(writer, context),
            Value::Map(map) => map.annotated_debuggable(self.mode).write_debug_for(writer, context),
            _ => AnnotatedDebuggable::new(self.value, self.mode).write_debug_for(writer, context),
        }
    }
}

use super::super::{super::annotation::*, value::*};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableValue
//

///
pub struct AnnotatedDebuggableValue<'own, AnnotationsT> {
    /// Value.
    pub value: &'own Value<AnnotationsT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotationsT> AnnotatedDebuggableValue<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotationsT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { value, mode }
    }
}

impl<'own, AnnotationsT> Debuggable for AnnotatedDebuggableValue<'own, AnnotationsT>
where
    AnnotationsT: Annotated,
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

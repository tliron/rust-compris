use super::{
    super::{super::annotation::*, list::*},
    value::*,
};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableList
//

///
pub struct AnnotatedDebuggableList<'own, AnnotatedT> {
    /// List.
    pub list: &'own List<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableList<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(list: &'own List<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { list, mode }
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
            self.list.into_iter().map(|value| AnnotatedDebuggableValue::new(value, self.mode)).collect();
        utils::write_debug_as_list(vector.iter(), None, writer, context)
    }
}

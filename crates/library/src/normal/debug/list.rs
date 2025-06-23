use super::{
    super::{super::annotation::*, list::*},
    value::*,
};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableList
//

///
pub struct AnnotatedDebuggableList<'own, AnnotationsT> {
    /// List.
    pub list: &'own List<AnnotationsT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotationsT> AnnotatedDebuggableList<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(list: &'own List<AnnotationsT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { list, mode }
    }
}

impl<'own, AnnotationsT> Debuggable for AnnotatedDebuggableList<'own, AnnotationsT>
where
    AnnotationsT: Annotated,
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

use super::{
    super::{super::annotation::*, map::*},
    value::*,
};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableMap
//

///
pub struct AnnotatedDebuggableMap<'own, AnnotationsT> {
    /// Map.
    pub map: &'own Map<AnnotationsT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotationsT> AnnotatedDebuggableMap<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(list: &'own Map<AnnotationsT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { map: list, mode }
    }
}

impl<'own, AnnotationsT> Debuggable for AnnotatedDebuggableMap<'own, AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> = self
            .map
            .into_iter()
            .map(|(key, value)| {
                (AnnotatedDebuggableValue::new(key, self.mode), AnnotatedDebuggableValue::new(value, self.mode))
            })
            .collect();
        let vector: Vec<_> = vector.iter().map(|(k, v)| (k, v)).collect();
        utils::write_debug_as_map(vector.into_iter(), None, writer, context)?;
        Ok(())
    }
}

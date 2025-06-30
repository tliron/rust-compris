use super::{
    super::{super::annotation::*, map::*},
    value::*,
};

use {kutil_cli::debug::*, std::io};

//
// AnnotatedDebuggableMap
//

///
pub struct AnnotatedDebuggableMap<'own, AnnotatedT> {
    /// Map.
    pub map: &'own Map<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, AnnotatedT> AnnotatedDebuggableMap<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(list: &'own Map<AnnotatedT>, mode: AnnotatedDebuggableMode) -> Self {
        Self { map: list, mode }
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

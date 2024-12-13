use super::{location::*, meta::*};

use {kutil_cli::debug::*, std::io};

//
// LocatedDebuggable
//

/// Provide a [Debuggable] implementation for any [Debuggable] with [HasMeta].
/// whereby the [Debuggable] is written first and the [Location] next.
pub struct LocatedDebuggable<'own, LocatableT>
where
    LocatableT: HasMeta,
{
    /// Debuggable.
    pub debuggable: &'own LocatableT,
}

impl<'own, LocatableT> LocatedDebuggable<'own, LocatableT>
where
    LocatableT: HasMeta + Debuggable,
{
    /// Constructor.
    pub fn new(debuggable: &'own LocatableT) -> Self {
        Self { debuggable }
    }
}

impl<'own, LocatableT> Debuggable for LocatedDebuggable<'own, LocatableT>
where
    LocatableT: HasMeta + Debuggable,
{
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        theme: &Theme,
    ) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        self.debuggable.write_debug_representation(writer, prefix, theme)?;
        self.debuggable.write_location_debug_representation(writer, prefix, theme)
    }
}

impl<'own, LocatableT> ToLocated<'own, LocatedDebuggable<'own, LocatableT>> for LocatableT
where
    LocatableT: HasMeta + Debuggable,
{
    fn to_located(&'own self) -> LocatedDebuggable<'own, LocatableT> {
        LocatedDebuggable::new(self)
    }
}

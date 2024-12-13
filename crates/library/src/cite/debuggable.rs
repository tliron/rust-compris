use super::citation::*;

use {kutil_cli::debug::*, std::io};

//
// CitedDebuggable
//

/// Provide a [Debuggable] implementation for any [Debuggable] with [Citable].
/// The [Citation] is written in the first line and the [Debuggable] in the next line.
pub struct CitedDebuggable<'own, CitableT>
where
    CitableT: Citable + Debuggable,
{
    /// Debuggable.
    pub debuggable: &'own CitableT,
}

impl<'own, CitableT> CitedDebuggable<'own, CitableT>
where
    CitableT: Citable + Debuggable,
{
    /// Constructor.
    pub fn new(citable_debuggable: &'own CitableT) -> Self {
        Self { debuggable: citable_debuggable }
    }
}

impl<'own, CitableT> Debuggable for CitedDebuggable<'own, CitableT>
where
    CitableT: Citable + Debuggable,
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
        let mut first = true;

        if let Some(citation) = self.debuggable.get_citation() {
            if citation.has_debug_representation() {
                citation.write_debug_representation(writer, prefix, theme)?;
                first = false;
            }
        }

        prefix.conditional_write(writer, first)?;
        self.debuggable.write_debug_representation(writer, prefix, theme)
    }
}

impl<'own, CitableT> ToCited<'own, CitedDebuggable<'own, CitableT>> for CitableT
where
    CitableT: Citable + Debuggable + 'own,
{
    fn to_cited(&'own self) -> CitedDebuggable<'own, CitableT> {
        CitedDebuggable::new(self)
    }
}

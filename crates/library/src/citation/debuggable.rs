use super::citation::*;

use {kutil_cli::debug::*, std::io};

//
// CitedDebuggable
//

/// Provide a [Debuggable] implementation for any [Citable] [Debuggable].
/// The [Citation] is written first and the [Debuggable] next.
pub struct CitedDebuggable<'a, CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    /// Citable debuggable.
    pub citable_debuggable: &'a CitableDebuggableT,
}

impl<'a, CitableDebuggableT> CitedDebuggable<'a, CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    /// Constructor.
    pub fn new(citable_debuggable: &'a CitableDebuggableT) -> Self {
        Self { citable_debuggable }
    }
}

impl<'a, CitableDebuggableT> Debuggable for CitedDebuggable<'a, CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let mut first = true;

        let citation = self.citable_debuggable.get_citation();
        if citation.has_debug_representation() {
            citation.write_debug_representation(writer, prefix, styles)?;
            first = false;
        }

        prefix.write(writer, first)?;
        self.citable_debuggable.write_debug_representation(writer, prefix, styles)
    }
}

// impl<'a, T: Citable + Debuggable> ToCited<'a, CitedDebuggable<'a, T>> for &'a T {
//     fn to_cited(&'a self) -> CitedDebuggable<'a, T> {
//         CitedDebuggable::new(self)
//     }
// }

impl<'a, CitableDebuggableT> ToCited<'a, CitedDebuggable<'a, CitableDebuggableT>> for CitableDebuggableT
where
    CitableDebuggableT: Citable + Debuggable + 'a,
{
    fn to_cited(&'a self) -> CitedDebuggable<'a, CitableDebuggableT> {
        CitedDebuggable::new(self)
    }
}

use super::citation::*;

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    owo_colors::*,
    std::{collections::*, io},
};

//
// CitedDebuggables
//

/// Provide a [Debuggable] implementation for a sequence of [Citable] [Debuggable].
/// The representation is divided into sections per source and each section is sorted by
/// citation location. Each item is represented as a [CitedDebuggable](super::debuggable::CitedDebuggable).
pub struct CitedDebuggables<'own, CitableT, IterableT>
where
    CitableT: Citable + Debuggable + 'own,
    &'own IterableT: IntoIterator<Item = &'own CitableT>,
{
    /// Citable debuggables.
    pub citable_debuggables: &'own IterableT,
}

impl<'own, CitableT, IterableT> CitedDebuggables<'own, CitableT, IterableT>
where
    CitableT: Citable + Debuggable + 'own,
    &'own IterableT: IntoIterator<Item = &'own CitableT>,
{
    /// Constructor.
    pub fn new(citable_debuggables: &'own IterableT) -> Self {
        Self { citable_debuggables }
    }
}

impl<'own, CitableT, IterableT> Debuggable for CitedDebuggables<'own, CitableT, IterableT>
where
    CitableT: Citable + Debuggable + 'own,
    &'own IterableT: IntoIterator<Item = &'own CitableT>,
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
        let mut table = HashMap::<_, Vec<_>>::new();
        for citable_debuggable in self.citable_debuggables {
            let source = match citable_debuggable.get_citation() {
                Some(citation) => citation.source.clone(),
                None => None,
            };

            match table.get_mut(&source) {
                Some(list) => list.push(citable_debuggable),
                None => {
                    let mut list = Vec::new();
                    list.push(citable_debuggable);
                    table.insert(source, list);
                }
            }
        }

        //wrapped.sort_by(|_a, _b| std::cmp::Ordering::Equal);

        for ((source, list), first) in IterateWithFirst::new(table) {
            let section = match source {
                Some(source) => source,
                None => "source".into(),
            };

            prefix.conditional_write(writer, first)?;
            write!(writer, "{}", section.style(theme.meta))?;

            for (citable_debuggable, last) in IterateWithLast::new(list) {
                prefix.write_with_branch(writer, last)?;
                citable_debuggable.to_cited().write_debug_representation(writer, &prefix.with_branch(last), theme)?;
            }
        }

        Ok(())
    }
}

impl<'own, CitableT, IterableT> ToCited<'own, CitedDebuggables<'own, CitableT, IterableT>> for IterableT
where
    CitableT: Citable + Debuggable + 'own,
    &'own IterableT: IntoIterator<Item = &'own CitableT>,
{
    fn to_cited(&'own self) -> CitedDebuggables<'own, CitableT, IterableT> {
        CitedDebuggables::new(self)
    }
}

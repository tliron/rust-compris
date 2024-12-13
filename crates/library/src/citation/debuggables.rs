use super::citation::*;

use {
    kutil_cli::debug::*,
    kutil_std::{borrow::*, error::*, iter::*},
    owo_colors::*,
    std::{borrow::*, collections::*, io},
};

//
// CitedDebuggables
//

///
pub struct CitedDebuggables<'a, CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    /// Citable debuggables.
    pub citable_debuggables: Calf<'a, Vec<CitableDebuggableT>>,
}

impl<'a, CitableDebuggableT> CitedDebuggables<'a, CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    /// Constructor.
    pub fn new(citable_debuggables: &'a Vec<CitableDebuggableT>) -> Self {
        Self { citable_debuggables: Calf::Borrowed(citable_debuggables) }
    }

    /// Constructor.
    pub fn new_from(citable_debuggables: impl IntoIterator<Item = CitableDebuggableT>) -> Self {
        let iterator = citable_debuggables.into_iter();
        let citable_debuggables = iterator.collect();
        Self { citable_debuggables: Calf::Owned(citable_debuggables) }
    }
}

impl<'a, CitableDebuggableT> Debuggable for CitedDebuggables<'a, CitableDebuggableT>
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
        let mut table = HashMap::<_, Vec<_>>::new();
        let citable_debuggables: &Vec<_> = self.citable_debuggables.borrow();
        for citable_debuggable in citable_debuggables {
            let source = match &citable_debuggable.get_citation().source {
                Some(source) => Some(source),
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

        for ((source, list), first) in FirstIterator::new(table) {
            let section = match source {
                Some(source) => source,
                None => "source",
            };

            prefix.write(writer, first)?;
            write!(writer, "{}", section.style(styles.meta))?;

            for (citable_debuggable, last) in LastIterator::new(list) {
                prefix.write_with_branch(writer, last)?;
                citable_debuggable.to_cited().write_debug_representation(writer, &prefix.with_branch(last), styles)?;
            }
        }

        Ok(())
    }
}

impl<'a, CitableDebuggableT> ToCited<'a, CitedDebuggables<'a, CitableDebuggableT>> for Vec<CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable,
{
    fn to_cited(&'a self) -> CitedDebuggables<'a, CitableDebuggableT> {
        CitedDebuggables::new(self)
    }
}

impl<'a, CitableDebuggableT> ToCited<'a, CitedDebuggables<'a, CitableDebuggableT>> for Errors<CitableDebuggableT>
where
    CitableDebuggableT: Citable + Debuggable + 'a,
{
    fn to_cited(&'a self) -> CitedDebuggables<'a, CitableDebuggableT> {
        CitedDebuggables::new(&self.errors)
    }
}

// TODO

// impl<'a, CitableDebuggableT, Iterable> ToCited<'a, CitedDebuggables<'a, CitableDebuggableT>> for Iterable
// where
//     CitableDebuggableT: Citable + Debuggable,
//     &'a Iterable: IntoIterator<Item = CitableDebuggableT> + 'a,
// {
//     fn to_cited(&'a self) -> CitedDebuggables<'a, CitableDebuggableT> {
//         CitedDebuggables::new_from(self)
//     }
// }

// impl<'a, CitableDebuggableT, Iterable> ToCited<'a, CitedDebuggables<'a, CitableDebuggableT>> for Iterable
// where
//     CitableDebuggableT: Citable + Debuggable,
//     &'a Iterable: IntoIterator<Item = CitableDebuggableT> + 'a,
// {
//     fn to_cited(&'a self) -> CitedDebuggables<'a, CitableDebuggableT> {
//         CitedDebuggables::new_from(self)
//     }
// }

// impl<'a, CitableDebuggableT> ToCited<'a, CitedDebuggables<'a, &'a CitableDebuggableT>>
//     for std::slice::Iter<'a, CitableDebuggableT>
// where
//     &'a CitableDebuggableT: Citable + Debuggable,
// {
//     fn to_cited(&'a self) -> CitedDebuggables<'a, &'a CitableDebuggableT> {
//         let v: Vec<_> = self.clone().collect();
//         CitedDebuggables::new(&v)
//     }
// }

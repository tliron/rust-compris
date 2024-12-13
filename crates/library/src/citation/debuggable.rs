use super::citation::*;

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{collections::*, io},
};

//
// AsDebuggableWithCitation.
//

/// Convert to a type that supports [Debuggable].
pub trait AsDebuggableWithCitation<T: Debuggable> {
    /// Convert to a type that supports [Debuggable].
    fn as_debuggable_with_citation(self) -> T;
}

//
// DebuggableWithCitation
//

/// Provide a [Debuggable] implementation for a [Debuggable] with [HasCitation].
pub struct DebuggableWithCitation<'a, T> {
    /// Wrapped.
    pub wrapped: &'a T,
}

impl<'a, T> DebuggableWithCitation<'a, T> {
    /// Constructor.
    pub fn new(wrapped: &'a T) -> Self {
        Self { wrapped }
    }
}

impl<'a, T: Debuggable + HasCitation> AsDebuggableWithCitation<DebuggableWithCitation<'a, T>> for &'a T {
    fn as_debuggable_with_citation(self) -> DebuggableWithCitation<'a, T> {
        DebuggableWithCitation::new(self)
    }
}

impl<'a, T: Debuggable + HasCitation> Debuggable for DebuggableWithCitation<'a, T> {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        nested_prefix: &NestedPrefix,
        styles: &Styles,
    ) -> io::Result<()> {
        let mut first = true;
        let citation = self.wrapped.get_citation();
        let has_path = citation.path.is_some();
        let has_coordinates = citation.coordinates.is_some();
        if has_path || has_coordinates {
            if let Some(path) = &citation.path {
                write!(writer, "{}", path.style(styles.meta))?;
            }

            if let Some(coordinates) = &citation.coordinates {
                if has_path {
                    write!(writer, " ")?;
                }
                coordinates.write_debug_representation(writer, nested_prefix, styles)?;
            }

            first = false;
        }

        nested_prefix.write(writer, first)?;
        self.wrapped.write_debug_representation(writer, nested_prefix, styles)
    }
}

//
// DebuggablesWithCitation
//

/// Provide a [Debuggable] implementation for a [Vec] of [Debuggable] with [HasCitation].
pub struct DebuggablesWithCitation<'a, T> {
    /// Wrapped errors.
    pub wrapped: &'a Vec<T>,
}

impl<'a, T> DebuggablesWithCitation<'a, T> {
    /// Constructor.
    pub fn new(wrapped: &'a Vec<T>) -> Self {
        Self { wrapped }
    }
}

impl<'a, E: Debuggable + HasCitation> AsDebuggableWithCitation<DebuggablesWithCitation<'a, E>> for &'a Vec<E> {
    fn as_debuggable_with_citation(self) -> DebuggablesWithCitation<'a, E> {
        DebuggablesWithCitation::new(self)
    }
}

impl<'a, E: Debuggable + HasCitation> Debuggable for DebuggablesWithCitation<'a, E> {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        nested_prefix: &NestedPrefix,
        styles: &Styles,
    ) -> io::Result<()> {
        let mut table = HashMap::<Option<&String>, Vec<&E>>::new();

        for element in self.wrapped {
            let source = match &element.get_citation().source {
                Some(source) => Some(source),
                None => None,
            };

            match table.get_mut(&source) {
                Some(list) => list.push(element),
                None => {
                    let mut list = Vec::new();
                    list.push(element);
                    table.insert(source, list);
                }
            }
        }

        //wrapped.sort_by(|_a, _b| std::cmp::Ordering::Equal);

        let mut first = true;
        for (source, list) in table {
            let section = match source {
                Some(source) => source,
                None => "source",
            };

            nested_prefix.write(writer, first)?;
            write!(writer, "{}", section.style(styles.meta))?;

            let mut i = list.iter().peekable();
            while let Some(element) = i.next() {
                let is_last = i.peek().is_none();
                nested_prefix.write_with_branch(writer, is_last)?;
                element.as_debuggable_with_citation().write_debug_representation(
                    writer,
                    &nested_prefix.with_branch(is_last),
                    styles,
                )?;
            }

            first = false;
        }

        Ok(())
    }
}

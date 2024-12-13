use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
};

//
// Coordinates
//

/// Coordinates metadata for a normal value.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Coordinates {
    /// Index.
    ///
    /// Note that it can be a byte index *or* a rune index,
    /// depending on the implementation.
    pub index: Option<usize>,

    /// Row and column.
    pub row_and_column: Option<(usize, usize)>,
}

impl Coordinates {
    /// Constructor.
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index: Some(index), row_and_column: Some((row, column)) }
    }
}

impl Debuggable for Coordinates {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _nested_prefix: &NestedPrefix,
        styles: &Styles,
    ) -> Result<(), std::io::Error> {
        write!(writer, "{} {}", "@".style(styles.delimiter), self.style(styles.meta))?;

        // if let Some(path) = &self.path {
        //     let indent = " ".repeat(indentation);
        //     write!(writer, "\n{}{}", indent, path.style(styles.meta))?;
        // }

        Ok(())
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((row, column)) = self.row_and_column {
            write!(formatter, "{},{}", row, column)?;
        }

        if let Some(index) = self.index {
            write!(formatter, "/{}", index)?;
        }

        Ok(())
    }
}

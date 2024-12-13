use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
};

//
// Coordinates
//

/// Coordinates metadata for normal values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Coordinates {
    /// Index.
    ///
    /// Note that it can be a byte index *or* a rune index,
    /// depending on the implementation.
    pub index: Option<usize>,

    /// Row and column.
    pub row_and_column: Option<(usize, Option<usize>)>,
}

impl Coordinates {
    /// Constructor.
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index: Some(index), row_and_column: Some((row, Some(column))) }
    }

    /// Returns [Coordinates::index] as isize,
    /// using -1 instead of [None].
    pub fn get_index_as_isize(&self) -> isize {
        match self.index {
            Some(index) => index as isize,
            None => -1,
        }
    }

    /// Returns [Coordinates::row_and_column] as a tuple of isize,
    /// using -1 instead of [None].
    pub fn get_row_and_column_as_isize(&self) -> (isize, isize) {
        match self.row_and_column {
            Some((row, column)) => match column {
                Some(column) => (row as isize, column as isize),
                None => (row as isize, -1),
            },
            None => (-1, -1),
        }
    }
}

impl Debuggable for Coordinates {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        styles: &Styles,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} {}", "@".style(styles.delimiter), self.style(styles.meta))?;
        Ok(())
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((row, column)) = self.row_and_column {
            if let Some(column) = column {
                write!(formatter, "{},{}", row, column)?;
            } else {
                write!(formatter, "{}", row)?;
            }
        }

        if let Some(index) = self.index {
            write!(formatter, "/{}", index)?;
        }

        Ok(())
    }
}

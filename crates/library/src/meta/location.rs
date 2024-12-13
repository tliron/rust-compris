use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
};

//
// Location
//

/// Location metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Location {
    /// Index.
    ///
    /// Note that it can be a byte index *or* a rune index,
    /// depending on the implementation.
    pub index: Option<usize>,

    /// Row and column.
    pub row_and_column: Option<(usize, Option<usize>)>,
}

impl Location {
    /// Constructor.
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index: Some(index), row_and_column: Some((row, Some(column))) }
    }

    /// Returns [Location::index](Location) as isize,
    /// using -1 instead of [None].
    pub fn get_index_as_isize(&self) -> isize {
        match self.index {
            Some(index) => index as isize,
            None => -1,
        }
    }

    /// Returns [Location::row_and_column](Location) as a tuple of isize,
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

impl Debuggable for Location {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        _prefix: &DebugPrefix,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} {}", "@".style(theme.delimiter), self.style(theme.meta))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((row, column)) = self.row_and_column {
            // Though our row and column start at 0, users usually expect them to start at 1
            if let Some(column) = column {
                write!(formatter, "{}:{}", row + 1, column + 1)?;
            } else {
                write!(formatter, "{}", row + 1)?;
            }
        }

        // if let Some(index) = self.index {
        //     write!(formatter, ";{}", index)?;
        // }

        Ok(())
    }
}

//
// ToLocated
//

/// Convert to a version with a [Location].
pub trait ToLocated<'own, LocatedT>
where
    LocatedT: 'own,
{
    /// Convert to version with a [Location].
    fn to_located(&'own self) -> LocatedT;
}

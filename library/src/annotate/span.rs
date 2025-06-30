use super::location::*;

use {
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// Span
//

/// Span annotation.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
    /// Start.
    pub start: Location,

    /// Optional end.
    pub end: Option<Location>,
}

impl Span {
    /// Constructor.
    pub fn new(start: Location, end: Option<Location>) -> Self {
        Self { start, end }
    }

    /// Whether [Debuggable] will have output.
    pub fn has_debug(&self) -> bool {
        self.start.has_debug()
    }
}

impl Debuggable for Span {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if self.start.has_debug() {
            self.start.write_debug_for(writer, context)?;

            if let Some(end) = &self.end
                && end.has_debug()
            {
                context.theme.write_delimiter(writer, "-")?;
                end.write_debug_for(writer, context)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.has_debug() {
            write!(formatter, "{}", self.start)?;

            if let Some(end) = &self.end {
                if end.has_debug() {
                    write!(formatter, "-{}", end)?;
                }
            }
        }

        Ok(())
    }
}

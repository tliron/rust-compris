use super::location::*;

use {
    kutil::cli::depict::*,
    std::{fmt, io},
};

/// Depict span separator.
pub const DEPICT_SPAN_SEPARATOR: char = '→';

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

    /// Whether [Depict] will have output.
    pub fn has_debug(&self) -> bool {
        self.start.has_debug()
    }
}

impl Depict for Span {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if self.start.has_debug() {
            self.start.depict(writer, context)?;

            if let Some(end) = &self.end
                && end.has_debug()
            {
                context.theme.write_delimiter(writer, DEPICT_SPAN_SEPARATOR)?;
                end.depict(writer, context)?;
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
                    write!(formatter, "{}{}", DEPICT_SPAN_SEPARATOR, end)?;
                }
            }
        }

        Ok(())
    }
}

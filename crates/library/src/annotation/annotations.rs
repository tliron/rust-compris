use super::{super::path::*, label::*, span::*};

use {
    bytestring::*,
    kutil_cli::debug::*,
    std::{fmt, io},
};

//
// Annotations
//

/// Annotations.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Annotations {
    /// Source.
    pub source: Option<ByteString>,

    /// Span.
    pub span: Option<Span>,

    /// Path.
    pub path: Option<PathRepresentation>,

    /// Label.
    pub label: Option<Label>,
}

impl Annotations {
    /// Whether [Debuggable] will have output.
    pub fn has_debug(&self, format: DebugFormat) -> bool {
        match format {
            DebugFormat::Verbose => {
                self.source.is_some()
                    || self.path.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }

            DebugFormat::Reduced => {
                self.path.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }

            DebugFormat::Compact => {
                self.source.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }
        }
    }
}

impl Debuggable for Annotations {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match context.format {
            // source + span + path
            DebugFormat::Verbose => {
                context.separate(writer)?;

                let mut separate = false;
                if let Some(source) = &self.source {
                    context.theme.write_meta(writer, source)?;
                    separate = true;
                }

                if let Some(span) = &self.span
                    && span.has_debug()
                {
                    if separate {
                        write!(writer, " ")?;
                    }
                    context.theme.write_delimiter(writer, "@")?;
                    span.write_debug_for(writer, context)?;
                    separate = true;
                }

                if let Some(path) = &self.path {
                    if separate {
                        write!(writer, " ")?;
                    }
                    path.write_debug_for(writer, context)?;
                }

                Ok(())
            }

            // path + span (no source)
            DebugFormat::Reduced => {
                context.separate(writer)?;

                let mut separate = false;
                if let Some(path) = &self.path {
                    path.write_debug_for(writer, context)?;
                    separate = true;
                }

                if let Some(span) = &self.span
                    && span.has_debug()
                {
                    if separate {
                        write!(writer, " ")?;
                    }
                    context.theme.write_delimiter(writer, "@")?;
                    span.write_debug_for(writer, context)?;
                }

                Ok(())
            }

            // source + span (no path)
            DebugFormat::Compact => {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "@")?;

                let mut separate = false;
                if let Some(source) = &self.source {
                    context.theme.write_meta(writer, source)?;
                    separate = true;
                }

                if let Some(span) = &self.span
                    && span.has_debug()
                {
                    if separate {
                        context.theme.write_delimiter(writer, ":")?;
                    }
                    span.write_debug_for(writer, context)?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Display for Annotations {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separate = false;
        if let Some(source) = &self.source {
            write!(formatter, "{}", source)?;
            separate = true;
        }

        if let Some(span) = &self.span {
            if separate {
                write!(formatter, " ")?;
            }
            write!(formatter, "@{}", span)?;
            separate = true;
        }

        if let Some(path) = &self.path {
            if separate {
                write!(formatter, " ")?;
            }
            write!(formatter, "{}", path)?;
        }

        Ok(())
    }
}

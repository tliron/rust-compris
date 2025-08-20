use super::{super::path::*, label::*, span::*};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::{fmt, io},
};

/// Depict annotations prefix.
pub const DEPICT_ANNOTATIONS_PREFIX: char = '@';

/// Depict annotations separator.
pub const DEPICT_ANNOTATIONS_SEPARATOR: char = ':';

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
    /// True if any field is [Some].
    pub fn has_some(&self) -> bool {
        self.source.is_some() || self.span.is_some() || self.path.is_some() || self.label.is_some()
    }

    /// Whether [Depict] will have output.
    pub fn has_depiction(&self, format: DepictionFormat) -> bool {
        match format {
            DepictionFormat::Compact => {
                self.source.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }

            DepictionFormat::Optimized => {
                self.path.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }

            DepictionFormat::Verbose => {
                self.source.is_some()
                    || self.path.is_some()
                    || match &self.span {
                        Some(span) => span.has_debug(),
                        None => false,
                    }
            }
        }
    }

    /// Set source.
    pub fn with_source(mut self, source: ByteString) -> Self {
        self.source = Some(source);
        self
    }

    /// Set span.
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Set path.
    pub fn with_path(mut self, path: PathRepresentation) -> Self {
        self.path = Some(path);
        self
    }

    /// Push list index to [Path].
    pub fn with_path_list_index(mut self, index: usize) -> Self {
        if let Some(path) = &mut self.path {
            path.push_list_index(index);
        }

        self
    }

    /// Push map key to [Path].
    pub fn with_path_map_key(mut self, key: ByteString) -> Self {
        if let Some(path) = &mut self.path {
            path.push_map_key(key);
        }

        self
    }

    /// Set label.
    pub fn with_label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

impl Depict for Annotations {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match context.get_format() {
            // source + span (no path)
            DepictionFormat::Compact => {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_PREFIX)?;

                let mut separate = false;
                if let Some(source) = &self.source {
                    context.theme.write_meta(writer, source)?;
                    separate = true;
                }

                if let Some(span) = &self.span
                    && span.has_debug()
                {
                    if separate {
                        context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_SEPARATOR)?;
                    }
                    span.depict(writer, context)?;
                }

                Ok(())
            }

            // source + span + path
            DepictionFormat::Verbose => {
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
                    context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_PREFIX)?;
                    span.depict(writer, context)?;
                    separate = true;
                }

                if let Some(path) = &self.path {
                    if separate {
                        write!(writer, " ")?;
                    }
                    path.depict(writer, context)?;
                }

                Ok(())
            }

            // path + span (no source)
            DepictionFormat::Optimized => {
                context.separate(writer)?;

                let mut separate = false;
                if let Some(path) = &self.path {
                    path.depict(writer, context)?;
                    separate = true;
                }

                if let Some(span) = &self.span
                    && span.has_debug()
                {
                    if separate {
                        write!(writer, " ")?;
                    }
                    context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_PREFIX)?;
                    span.depict(writer, context)?;
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
            write!(formatter, "{}{}", DEPICT_ANNOTATIONS_PREFIX, span)?;
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

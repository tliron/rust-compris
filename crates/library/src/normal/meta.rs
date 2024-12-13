use super::normal::*;

use {
    kutil_cli::debug::*,
    owo_colors::*,
    std::{fmt, io},
};

//
// Meta
//

/// Optional metadata associated with normal values.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    /// Optional location metadata.
    pub location: Option<Location>,

    /// Optional annotation metadata.
    pub annotation: Option<Annotation>,
}

impl Meta {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set location metadata.
    pub fn with_location(mut self, location: Option<Location>) -> Self {
        self.location = location;
        self
    }

    /// Set annotation metadata.
    pub fn with_annotation(mut self, annotation: Option<Annotation>) -> Self {
        self.annotation = annotation;
        self
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some(location) => location.fmt(formatter),
            None => Ok(()),
        }
    }
}

//
// MetaHelpers
//

/// Convenience functions for modifying metadata.
pub trait MetaHelpers {
    /// Sets the metadata.
    fn with_meta(self, meta: Meta) -> Self;

    /// Sets the location metadata.
    fn with_location(self, location: Option<Location>) -> Self;

    /// Sets the annotation metadata.
    fn with_annotation(self, annotation: Option<Annotation>) -> Self;

    /// Sets the annotation metadata as an integer.
    fn with_annotation_integer(self, annotation: i64) -> Self;

    /// Sets the annotation metadata as a string.
    fn with_annotation_string(self, annotation: String) -> Self;
}

impl<T: Normal> MetaHelpers for T {
    fn with_meta(mut self, meta: Meta) -> Self {
        if let Some(self_meta) = self.get_meta_mut() {
            *self_meta = meta;
        }
        self
    }

    fn with_location(mut self, location: Option<Location>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.location = location;
        }
        self
    }

    fn with_annotation(mut self, annotation: Option<Annotation>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.annotation = annotation;
        }
        self
    }

    fn with_annotation_integer(self, annotation: i64) -> Self {
        self.with_annotation(Some(Annotation::Integer(annotation)))
    }

    fn with_annotation_string(self, annotation: String) -> Self {
        self.with_annotation(Some(Annotation::String(annotation)))
    }
}

//
// Annotation
//

/// Annotation metadata for a normal value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Annotation {
    /// Integer annotation.
    Integer(i64),

    /// String annotation.
    String(String),
}

//
// Location
//

/// Location metadata for a normal value.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Location {
    /// Index.
    ///
    /// Note that it can be a byte index *or* a rune index,
    /// depending on the implementation.
    pub index: Option<usize>,

    /// Row and column.
    pub row_and_column: Option<(usize, usize)>,
}

impl Location {
    /// Constructor.
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index: Some(index), row_and_column: Some((row, column)) }
    }
}

impl WriteDebug for Location {
    fn write_debug_representation<W: io::Write>(
        &self,
        writer: &mut W,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), std::io::Error> {
        write!(writer, "{}", format!("@{}", self).style(styles.meta))?;

        // if let Some(path) = &self.path {
        //     let indent = " ".repeat(indentation);
        //     write!(writer, "\n{}{}", indent, path.style(styles.meta))?;
        // }

        Ok(())
    }
}

impl fmt::Display for Location {
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

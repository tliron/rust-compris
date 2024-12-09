use super::{
    super::{styles::*, write_debug::*},
    normal::*,
};

use {
    owo_colors::OwoColorize,
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
    pub fn with_location(&mut self, location: Option<Location>) -> &mut Self {
        self.location = location;
        self
    }

    /// Set annotation metadata.
    pub fn with_annotation(&mut self, annotation: Option<Annotation>) -> &mut Self {
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
    fn with_meta(self, meta: &Meta) -> Self;

    /// Sets the location metadata.
    fn with_location_option(self, location: Option<Location>) -> Self;

    /// Sets the location metadata.
    fn with_location(self, location: Location) -> Self;

    /// Sets the annotation metadata.
    fn with_annotation_option(self, annotation: Option<Annotation>) -> Self;

    /// Sets the annotation metadata.
    fn with_annotation(self, annotation: Annotation) -> Self;

    /// Sets the annotation metadata as an integer.
    fn with_annotation_integer(self, annotation: i64) -> Self;

    /// Sets the annotation metadata as a string.
    fn with_annotation_string(self, annotation: String) -> Self;
}

impl<T: Normal> MetaHelpers for T {
    fn with_meta(mut self, meta: &Meta) -> Self {
        if let Some(self_meta) = self.get_meta_mut() {
            *self_meta = meta.clone();
        }
        self
    }

    fn with_location_option(mut self, location: Option<Location>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.with_location(location);
        }
        self
    }

    fn with_location(self, location: Location) -> Self {
        self.with_location_option(Some(location))
    }

    fn with_annotation_option(mut self, annotation: Option<Annotation>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.with_annotation(annotation);
        }
        self
    }

    fn with_annotation(self, annotation: Annotation) -> Self {
        self.with_annotation_option(Some(annotation))
    }

    fn with_annotation_integer(self, annotation: i64) -> Self {
        self.with_annotation(Annotation::Integer(annotation))
    }

    fn with_annotation_string(self, annotation: String) -> Self {
        self.with_annotation(Annotation::String(annotation))
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
    pub index: usize,

    /// Row.
    pub row: usize,

    /// Column.
    pub column: usize,
}

impl Location {
    /// Constructor.
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index, row, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}:{}", self.row, self.column, self.index)
    }
}

impl<W: io::Write> WriteDebug<W> for Location {
    fn write_debug_representation(
        &self,
        writer: &mut W,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), std::io::Error> {
        write!(writer, " @{}", self.style(styles.meta))
    }
}

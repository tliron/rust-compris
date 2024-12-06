use super::{styles::*, write_debug::*};

use {
    owo_colors::OwoColorize,
    std::{fmt, io},
};

//
// Meta
//

/// ARD metadata.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    pub location: Option<Location>,
    pub annotation: Option<Annotation>,
}

impl Meta {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_location(&mut self, location: Option<Location>) -> &mut Self {
        self.location = location;
        self
    }

    pub fn with_annotation(&mut self, annotation: Option<Annotation>) -> &mut Self {
        self.annotation = annotation;
        self
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some(location) => write!(formatter, "{}", location),
            None => Ok(()),
        }
    }
}

//
// HasMeta
//

pub trait HasMeta {
    fn get_meta(&self) -> &Meta;
    fn get_meta_mut(&mut self) -> &mut Meta;
}

//
// MetaHelpers
//

pub trait MetaHelpers {
    fn with_meta_clone(self, meta: &Meta) -> Self;
    fn with_location_option(self, location: Option<Location>) -> Self;
    fn with_location(self, location: Location) -> Self;
    fn with_annotation_option(self, annotation: Option<Annotation>) -> Self;
    fn with_annotation(self, annotation: Annotation) -> Self;
    fn with_annotation_integer(self, annotation: i64) -> Self;
    fn with_annotation_string(self, annotation: String) -> Self;
}

impl<T: HasMeta> MetaHelpers for T {
    fn with_meta_clone(mut self, meta: &Meta) -> Self {
        *self.get_meta_mut() = meta.clone();
        self
    }

    fn with_location_option(mut self, location: Option<Location>) -> Self {
        self.get_meta_mut().with_location(location);
        self
    }

    fn with_location(self, location: Location) -> Self {
        self.with_location_option(Some(location))
    }

    fn with_annotation_option(mut self, annotation: Option<Annotation>) -> Self {
        self.get_meta_mut().with_annotation(annotation);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Annotation {
    Integer(i64),
    String(String),
}

//
// Location
//

/// ARD location.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Location {
    pub index: usize,
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub fn new(index: usize, row: usize, column: usize) -> Self {
        Self { index, row, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}:{}", self.row, self.column, self.index)
    }
}

impl WriteDebug for Location {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        _indentation: usize,
        styles: &Styles,
    ) -> Result<(), std::io::Error> {
        write!(writer, " @{}", self.style(styles.location))
    }
}

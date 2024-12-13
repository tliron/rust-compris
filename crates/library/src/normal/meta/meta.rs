use super::{super::normal::*, annotation::*, coordinates::*};

use std::fmt;

//
// Meta
//

/// Optional metadata associated with normal values.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    /// Optional coordinates metadata.
    pub coordinates: Option<Coordinates>,

    /// Optional annotation metadata.
    pub annotation: Option<Annotation>,
}

impl Meta {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coordinates metadata.
    pub fn with_coordinates(mut self, coordinates: Option<Coordinates>) -> Self {
        self.coordinates = coordinates;
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
        match &self.coordinates {
            Some(coordinates) => coordinates.fmt(formatter),
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

    /// Sets the metadata.
    fn with_meta_from<T: Normal>(self, other: &T) -> Self;

    /// Sets the coordinates metadata.
    fn with_coordinates(self, coordinates: Option<Coordinates>) -> Self;

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

    fn with_meta_from<O: Normal>(self, other: &O) -> Self {
        match other.get_meta() {
            Some(meta) => self.with_meta(meta.clone()),
            None => self,
        }
    }

    fn with_coordinates(mut self, coordinates: Option<Coordinates>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.coordinates = coordinates;
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

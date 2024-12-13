use super::{annotation::*, location::*};

use {kutil_cli::debug::*, std::fmt, std::io};

//
// Meta
//

/// Metadata.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
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

    /// Write the location debug representation if it exists.
    fn write_location_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.location {
            Some(location) => location.write_debug_for(writer, context),

            None => Ok(()),
        }
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some(location) => fmt::Display::fmt(location, formatter),
            None => Ok(()),
        }
    }
}

//
// HasMeta
//

/// Has [Meta].
pub trait HasMeta: Sized {
    /// Access to the metadata.
    fn get_meta(&self) -> Option<&Meta>;

    /// Mutable access to the metadata.
    fn get_meta_mut(&mut self) -> Option<&mut Meta>;

    /// Sets the metadata.
    fn with_meta(mut self, meta: Meta) -> Self {
        if let Some(self_meta) = self.get_meta_mut() {
            *self_meta = meta;
        }
        self
    }

    /// Sets the metadata.
    fn with_meta_from<OtherHasMetaT>(self, other: &OtherHasMetaT) -> Self
    where
        OtherHasMetaT: HasMeta,
    {
        match other.get_meta() {
            Some(meta) => self.with_meta(meta.clone()),
            None => self,
        }
    }

    /// Sets the location metadata.
    fn with_location(mut self, location: Option<Location>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.location = location;
        }
        self
    }

    /// Sets the annotation metadata.
    fn with_annotation(mut self, annotation: Option<Annotation>) -> Self {
        if let Some(meta) = self.get_meta_mut() {
            meta.annotation = annotation;
        }
        self
    }

    /// Sets the annotation metadata as an integer.
    fn with_annotation_integer(self, annotation: i64) -> Self {
        self.with_annotation(Some(Annotation::Integer(annotation)))
    }

    /// Sets the annotation metadata as a string.
    fn with_annotation_string(self, annotation: String) -> Self {
        self.with_annotation(Some(Annotation::String(annotation)))
    }

    /// Write the location debug representation if it exists.
    fn write_location_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.get_meta() {
            Some(meta) => meta.write_location_debug_for(writer, context),
            None => Ok(()),
        }
    }
}

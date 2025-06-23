use super::{super::path::*, annotations::*, label::*, span::*};

use bytestring::*;

//
// Annotated
//

/// Has annotations.
pub trait Annotated: Sized {
    /// Whether we have [Annotations].
    ///
    /// When false, the other functions are guaranteed to return [None], allowing for optimizing by
    /// avoiding unnecessary annotation construction.
    fn is_annotated() -> bool;

    /// Get [Annotations].
    fn get_annotations(&self) -> Option<&Annotations>;

    /// Get [Annotations] as mutable.
    fn get_annotations_mut(&mut self) -> Option<&mut Annotations>;

    /// Sets the [Annotations].
    fn set_annotations(&mut self, annotations: Annotations);

    /// Set [Annotations].
    fn with_annotations(mut self, annotations: Annotations) -> Self {
        self.set_annotations(annotations);
        self
    }

    /// Clone [Annotations] from another [Annotated].
    fn with_annotations_from<AnnotatedT>(mut self, source: &AnnotatedT) -> Self
    where
        AnnotatedT: Annotated,
    {
        if Self::is_annotated()
            && let Some(annotations) = source.get_annotations()
        {
            self.set_annotations(annotations.clone());
        }

        self
    }

    /// Set source.
    fn with_source(mut self, source: &Option<ByteString>) -> Self {
        if Self::is_annotated()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.source = source.clone();
        }

        self
    }

    /// Set [Path].
    fn with_path(mut self, path: Option<PathRepresentation>) -> Self {
        if Self::is_annotated()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.path = path;
        }

        self
    }

    /// Set [Span].
    fn with_span(mut self, span: Option<Span>) -> Self {
        if Self::is_annotated()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.span = span;
        }

        self
    }

    /// Set [Label]
    fn with_label(mut self, label: Option<Label>) -> Self {
        if Self::is_annotated()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.label = label;
        }

        self
    }
}

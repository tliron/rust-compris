use super::{super::path::*, annotations::*, label::*, maybe::*, span::*, r#struct::*};

use kutil::std::immutable::*;

//
// Annotated
//

/// Potentially has annotations.
pub trait Annotated
where
    Self: Sized,
{
    /// Whether we can have [Annotations].
    ///
    /// When false, the other trait functions are guaranteed to be no-ops.
    fn can_have_annotations() -> bool;

    /// The annotations.
    fn annotations(&self) -> Option<&Annotations>;

    /// The annotations as mutable.
    fn annotations_mut(&mut self) -> Option<&mut Annotations>;

    /// Whether we have [Annotations].
    fn has_annotations(&self) -> bool {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations()
        {
            annotations.has_some()
        } else {
            false
        }
    }

    /// Create a [MaybeAnnotations] with our [Annotations].
    fn maybe_annotations(&self) -> MaybeAnnotations<Self> {
        MaybeAnnotations::new_from(self)
    }

    /// Set annotations.
    fn with_annotations(mut self, annotations: Annotations) -> Self {
        if Self::can_have_annotations()
            && let Some(self_annotations) = self.annotations_mut()
        {
            *self_annotations = annotations;
        }

        self
    }

    /// Clone [Annotations] from another [Annotated].
    fn with_annotations_from<AnnotatedT>(mut self, source: &AnnotatedT) -> Self
    where
        AnnotatedT: Annotated,
    {
        if Self::can_have_annotations()
            && let Some(annotations) = source.annotations()
            && let Some(self_annotations) = self.annotations_mut()
        {
            *self_annotations = annotations.clone();
        }

        self
    }

    /// Clone [Annotations] from another [AnnotatedFields].
    fn with_annotations_from_field<AnnotatedFieldsT>(mut self, source: &AnnotatedFieldsT, name: &str) -> Self
    where
        AnnotatedFieldsT: AnnotatedStruct,
    {
        if Self::can_have_annotations()
            && let Some(annotations) = source.field_annotations(name)
            && let Some(self_annotations) = self.annotations_mut()
        {
            *self_annotations = annotations.clone();
        }

        self
    }

    /// Set source.
    fn with_source(mut self, source: &Option<ByteString>) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
        {
            annotations.source = source.clone();
        }

        self
    }

    /// Set path.
    fn with_path(mut self, path: Option<PathRepresentation>) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
        {
            annotations.path = path;
        }

        self
    }

    /// Push list index to path.
    fn with_path_list_index(mut self, index: usize) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
            && let Some(path) = &mut annotations.path
        {
            path.push_list_index(index);
        }

        self
    }

    /// Push map key to path.
    fn with_path_map_key(mut self, key: ByteString) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
            && let Some(path) = &mut annotations.path
        {
            path.push_map_key(key);
        }

        self
    }

    /// Set span.
    fn with_span(mut self, span: Option<Span>) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
        {
            annotations.span = span;
        }

        self
    }

    /// Set label.
    fn with_label(mut self, label: Option<Label>) -> Self {
        if Self::can_have_annotations()
            && let Some(annotations) = self.annotations_mut()
        {
            annotations.label = label;
        }

        self
    }
}

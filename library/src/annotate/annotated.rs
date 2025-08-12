use super::{super::path::*, annotations::*, label::*, span::*, r#struct::*};

use kutil::std::zerocopy::*;

//
// Annotated
//

/// Potentially has annotations.
pub trait Annotated
where
    Self: Sized,
{
    /// Whether we have [Annotations].
    ///
    /// When false, the other trait functions are guaranteed to be no-ops.
    fn has_annotations() -> bool;

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
        if Self::has_annotations()
            && let Some(annotations) = source.get_annotations()
        {
            self.set_annotations(annotations.clone());
        }

        self
    }

    /// Clone [Annotations] from another [AnnotatedFields].
    fn with_annotations_from_field<AnnotatedFieldsT>(mut self, source: &AnnotatedFieldsT, name: &str) -> Self
    where
        AnnotatedFieldsT: AnnotatedStruct,
    {
        if Self::has_annotations()
            && let Some(annotations) = source.get_field_annotations(name)
        {
            self.set_annotations(annotations.clone());
        }

        self
    }

    /// Set source.
    fn with_source(mut self, source: &Option<ByteString>) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.source = source.clone();
        }

        self
    }

    /// Set [Path].
    fn with_path(mut self, path: Option<PathRepresentation>) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.path = path;
        }

        self
    }

    /// Push list index to [Path].
    fn with_path_list_index(mut self, index: usize) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
            && let Some(path) = &mut annotations.path
        {
            path.push_list_index(index);
        }

        self
    }

    /// Push map key to [Path].
    fn with_path_map_key(mut self, key: ByteString) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
            && let Some(path) = &mut annotations.path
        {
            path.push_map_key(key);
        }

        self
    }

    /// Set [Span].
    fn with_span(mut self, span: Option<Span>) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.span = span;
        }

        self
    }

    /// Set [Label]
    fn with_label(mut self, label: Option<Label>) -> Self {
        if Self::has_annotations()
            && let Some(annotations) = self.get_annotations_mut()
        {
            annotations.label = label;
        }

        self
    }
}

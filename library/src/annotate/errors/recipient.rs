use super::super::{annotated::*, annotations::*, r#struct::*};

use kutil::std::error::*;

//
// AnnotatedErrorRecipient
//

/// An [ErrorRecipient] wrapper that adds an [Annotations] to errors that don't already have
/// [Annotations].
pub struct AnnotatedErrorRecipient<'own, InnerT> {
    /// Inner.
    pub inner: &'own mut InnerT,

    /// Annotations.
    pub annotations: Option<&'own Annotations>,
}

impl<'own, InnerT> AnnotatedErrorRecipient<'own, InnerT> {
    /// Constructor.
    pub fn new(inner: &'own mut InnerT, annotations: Option<&'own Annotations>) -> Self {
        Self { inner, annotations }
    }
}

impl<'own, ErrorT, InnerT> ErrorRecipient<ErrorT> for AnnotatedErrorRecipient<'own, InnerT>
where
    ErrorT: Annotated,
    InnerT: ErrorRecipient<ErrorT>,
{
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        if !error.has_annotations()
            && let Some(annotations) = self.annotations
        {
            self.inner.give_error(error.with_annotations(annotations.clone()))
        } else {
            self.inner.give_error(error)
        }
    }
}

//
// AnnotateErrors
//

/// Wrap in an [AnnotatedErrorRecipient].
pub trait AnnotateErrors<'own, ErrorT, InnerT> {
    /// Wrap in an [AnnotatedErrorRecipient].
    fn with_annotations(
        &'own mut self,
        annotations: Option<&'own Annotations>,
    ) -> AnnotatedErrorRecipient<'own, InnerT>;

    /// Wrap in an [AnnotatedErrorRecipient].
    fn with_field_annotations<StructT>(
        &'own mut self,
        r#struct: &'own StructT,
        name: &str,
    ) -> AnnotatedErrorRecipient<'own, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_annotations(r#struct.field_annotations(name))
    }
}

impl<'own, ErrorT, ErrorRecipientT> AnnotateErrors<'own, ErrorT, ErrorRecipientT> for ErrorRecipientT
where
    ErrorT: Annotated,
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    fn with_annotations(
        &'own mut self,
        annotations: Option<&'own Annotations>,
    ) -> AnnotatedErrorRecipient<'own, ErrorRecipientT> {
        AnnotatedErrorRecipient::new(self, annotations)
    }
}

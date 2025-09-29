use super::{annotated::*, annotations::*};

use std::{hash::*, marker::*};

//
// MaybeAnnotations
//

/// Maybe has [Annotations].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MaybeAnnotations<AnnotatedT> {
    /// Annotations.
    pub annotations: Option<Annotations>,

    annotated: PhantomData<AnnotatedT>,
}

impl<AnnotatedT> MaybeAnnotations<AnnotatedT> {
    /// Constructor.
    pub fn new(annotations: Option<Annotations>) -> Self {
        Self { annotations, annotated: PhantomData }
    }

    /// Constructor.
    pub fn new_from(annotated: &AnnotatedT) -> Self
    where
        AnnotatedT: Annotated,
    {
        if AnnotatedT::can_have_annotations()
            && let Some(annotations) = annotated.annotations()
        {
            Self::new(Some(annotations.clone()))
        } else {
            Self::default()
        }
    }
}

impl<AnnotatedT> Default for MaybeAnnotations<AnnotatedT> {
    fn default() -> Self {
        Self { annotations: None, annotated: PhantomData }
    }
}

impl<AnnotatedT> Annotated for MaybeAnnotations<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.annotations.as_ref()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotations.as_mut()
    }
}

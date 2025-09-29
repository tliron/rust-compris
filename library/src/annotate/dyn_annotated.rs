use super::{annotated::*, annotations::*};

//
// DynAnnotated
//

/// A reduced `dyn`-compatible version of [Annotated](super::annotated::Annotated).
pub trait DynAnnotated {
    /// The annotations.
    fn dyn_annotations(&self) -> Option<&Annotations>;

    /// The annotations as mutable.
    fn dyn_annotations_mut(&mut self) -> Option<&mut Annotations>;
}

impl<AnnotatedT> DynAnnotated for AnnotatedT
where
    AnnotatedT: Annotated,
{
    fn dyn_annotations(&self) -> Option<&Annotations> {
        self.annotations()
    }

    fn dyn_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotations_mut()
    }
}

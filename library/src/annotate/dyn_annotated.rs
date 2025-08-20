use super::{annotated::*, annotations::*};

//
// DynAnnotated
//

/// A reduced `dyn`-compatible version of [Annotated](super::annotated::Annotated).
pub trait DynAnnotated {
    /// Get [Annotations].
    fn dyn_get_annotations(&self) -> Option<&Annotations>;

    /// Get [Annotations] as mutable.
    fn dyn_get_annotations_mut(&mut self) -> Option<&mut Annotations>;
}

impl<AnnotatedT> DynAnnotated for AnnotatedT
where
    AnnotatedT: Annotated,
{
    fn dyn_get_annotations(&self) -> Option<&Annotations> {
        self.get_annotations()
    }

    fn dyn_get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.get_annotations_mut()
    }
}

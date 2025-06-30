use super::annotations::*;

//
// DynAnnotated
//

/// A reduced `dyn`-compatible version of [Annotated](super::annotated::Annotated).
pub trait DynAnnotated {
    /// Get [Annotations].
    fn get_annotations(&self) -> Option<&Annotations>;

    /// Get [Annotations] as mutable.
    fn get_annotations_mut(&mut self) -> Option<&mut Annotations>;

    /// Sets the [Annotations].
    fn set_annotations(&mut self, annotations: Annotations);
}

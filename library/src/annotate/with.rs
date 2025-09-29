use super::{annotated::*, annotations::*};

use std::hash::*;

//
// WithAnnotations
//

/// With [Annotations].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithAnnotations {
    /// Annotations.
    pub annotations: Annotations,
}

impl Annotated for WithAnnotations {
    fn can_have_annotations() -> bool {
        true
    }

    fn annotations(&self) -> Option<&Annotations> {
        Some(&self.annotations)
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        Some(&mut self.annotations)
    }
}

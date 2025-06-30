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
    fn has_annotations() -> bool {
        true
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        Some(&self.annotations)
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        Some(&mut self.annotations)
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        self.annotations = annotations
    }
}

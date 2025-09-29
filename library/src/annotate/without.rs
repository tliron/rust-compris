use super::{annotated::*, annotations::*};

use std::hash::*;

//
// WithoutAnnotations
//

/// Without [Annotations].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithoutAnnotations;

impl Annotated for WithoutAnnotations {
    fn can_have_annotations() -> bool {
        false
    }

    fn annotations(&self) -> Option<&Annotations> {
        None
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        None
    }
}

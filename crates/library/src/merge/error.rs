use super::super::normal::*;

use std::{error::*, fmt};

//
// MergeError
//

/// Merge error.
#[derive(Clone, Debug)]
pub struct MergeError<'own, AnnotationsT> {
    /// Cause of the error.
    pub cause: &'own Value<AnnotationsT>,
}

impl<'own, AnnotationsT> MergeError<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(cause: &'own Value<AnnotationsT>) -> Self {
        Self { cause }
    }
}

impl<'own, AnnotationsT> fmt::Display for MergeError<'own, AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "merge: {}", self.cause)
    }
}

impl<'own, AnnotationsT> Error for MergeError<'own, AnnotationsT> where AnnotationsT: fmt::Debug {}

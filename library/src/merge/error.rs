use super::super::normal::*;

use std::{error::*, fmt};

//
// MergeError
//

/// Merge error.
#[derive(Clone, Debug)]
pub struct MergeError<'own, AnnotatedT> {
    /// Cause of the error.
    pub cause: &'own Variant<AnnotatedT>,
}

impl<'own, AnnotatedT> MergeError<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(cause: &'own Variant<AnnotatedT>) -> Self {
        Self { cause }
    }
}

impl<'own, AnnotatedT> fmt::Display for MergeError<'own, AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "merge: {}", self.cause)
    }
}

impl<'own, AnnotatedT> Error for MergeError<'own, AnnotatedT> where AnnotatedT: fmt::Debug {}

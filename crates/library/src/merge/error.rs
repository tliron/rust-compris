use super::super::normal::*;

//
// MergeError
//

/// Merge error.
#[derive(Debug)]
pub struct MergeError<'own> {
    /// Cause of the error.
    pub cause: &'own Value,
}

impl<'own> MergeError<'own> {
    /// Constructor.
    pub fn new(cause: &'own Value) -> Self {
        Self { cause }
    }
}

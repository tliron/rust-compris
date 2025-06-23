use super::{super::normal::*, error::*, mode::*};

use {kutil_std::error::*, std::fmt};

impl<AnnotationsT> Value<AnnotationsT> {
    /// Merge another value into this value. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_errors<'own, ErrorRecipientT>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own, AnnotationsT>>
    where
        AnnotationsT: Clone,
        ErrorRecipientT: ErrorRecipient<MergeError<'own, AnnotationsT>>,
    {
        match self {
            Self::List(list) => match other {
                Self::List(other_list) => list.merge_with_errors(other_list, merge_mode, errors),
                _ => Ok(false),
            },

            Self::Map(map) => match other {
                Self::Map(other_map) => map.merge_with_errors(other_map, merge_mode, errors),
                _ => Ok(false),
            },

            _ => Ok(false),
        }
    }

    /// Merge another value into this value while failing on the first encountered error.
    /// Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_mode<'own>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
    ) -> Result<bool, MergeError<'own, AnnotationsT>>
    where
        AnnotationsT: Clone,
    {
        self.merge_with_errors(other, merge_mode, &mut FailFastErrorRecipient)
    }

    /// Merge another value into this value. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotationsT: Clone + fmt::Debug,
    {
        // The default mode should never cause errors, so unwrap is safe
        self.merge_with_mode(other, &MergeMode::default()).expect("merge_with_mode")
    }
}

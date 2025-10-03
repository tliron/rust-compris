use super::{super::normal::*, error::*, mode::*};

use {kutil::std::error::*, std::fmt};

impl<AnnotatedT> Variant<AnnotatedT> {
    /// Merge another [Variant] into this [Variant]. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_errors<'own, ErrorRecipientT>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own, AnnotatedT>>
    where
        AnnotatedT: Clone,
        ErrorRecipientT: ErrorRecipient<MergeError<'own, AnnotatedT>>,
    {
        match (self, other) {
            (Self::List(list), Self::List(other_list)) => list.merge_with_errors(other_list, merge_mode, errors),
            (Self::Map(map), Self::Map(other_map)) => map.merge_with_errors(other_map, merge_mode, errors),
            _ => Ok(false),
        }
    }

    /// Merge another [Variant] into this [Variant] while failing on the first encountered error.
    /// Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_mode<'own>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
    ) -> Result<bool, MergeError<'own, AnnotatedT>>
    where
        AnnotatedT: Clone,
    {
        self.merge_with_errors(other, merge_mode, &mut FailFastErrorRecipient)
    }

    /// Merge another [Variant] into this value. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotatedT: Clone + fmt::Debug,
    {
        // The default mode should never cause errors, so unwrap is safe
        self.merge_with_mode(other, &Default::default()).expect("merge_with_mode")
    }
}

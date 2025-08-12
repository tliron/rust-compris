use super::{super::normal::*, error::*, mode::*};

use {kutil::std::error::*, std::fmt};

impl<AnnotatedT> List<AnnotatedT> {
    /// Merge another list into this list. Return true if any change happened.
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
        match merge_mode.list {
            ListMergeMode::Append => {
                if other.inner.is_empty() {
                    Ok(false)
                } else {
                    self.inner.extend(other.inner.iter().cloned());
                    Ok(true)
                }
            }

            ListMergeMode::SkipExisting => {
                let mut changed = false;

                for item in &other.inner {
                    if self.push_unique_clone(item) {
                        changed = true;
                    }
                }

                Ok(changed)
            }

            ListMergeMode::FailExisting => {
                let mut changed = false;

                for item in &other.inner {
                    if self.push_unique_clone(item) {
                        changed = true;
                    } else {
                        errors.give(MergeError::new(item))?;
                    }
                }

                Ok(changed)
            }

            ListMergeMode::Replace => {
                if self == other {
                    Ok(false)
                } else {
                    self.inner = other.inner.clone();
                    Ok(true)
                }
            }
        }
    }

    /// Merge another list into this list while failing on the first encountered error.
    /// Return true if any change happened.
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

    /// Merge another list into this list. Return true if any change happened.
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

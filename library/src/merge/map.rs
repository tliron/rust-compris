use super::{super::normal::*, error::*, mode::*};

use {kutil_std::error::*, std::fmt};

impl<AnnotatedT> Map<AnnotatedT> {
    /// Merge another map into this map. Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_errors<'own, ErrorRecipientT>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own, AnnotatedT>>
    where
        Self: 'own,
        AnnotatedT: Clone,
        ErrorRecipientT: ErrorRecipient<MergeError<'own, AnnotatedT>>,
    {
        let mut changed = false;

        for (other_key, other_value) in &other.inner {
            if self.merge_key(other_key, other_value, merge_mode, errors)? {
                changed = true;
            }
        }

        Ok(changed)
    }

    /// Merge another map into this map while failing on the first encountered error.
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

    /// Merge another map into this map. Return true if any change happened.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotatedT: Clone + fmt::Debug,
    {
        // The default mode should never cause errors, so unwrap is safe
        self.merge_with_mode(other, &Default::default()).expect("merge_with_mode")
    }

    fn merge_key<'own, ErrorRecipientT>(
        &mut self,
        other_key: &'own Variant<AnnotatedT>,
        other_value: &'own Variant<AnnotatedT>,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own, AnnotatedT>>
    where
        AnnotatedT: Clone,
        ErrorRecipientT: ErrorRecipient<MergeError<'own, AnnotatedT>>,
    {
        match self.inner.get_mut(other_key) {
            Some(value) => {
                // We already have the key, so merge the value
                Ok(value.merge_with_errors(other_value, merge_mode, errors)?)
            }

            None => {
                // We don't have the key, so insert it
                if self.inner.insert(other_key.clone(), other_value.clone()).is_some() {
                    if merge_mode.map == MapMergeMode::FailExisting {
                        errors.give(MergeError::new(other_key))?;
                    }
                }
                Ok(true)
            }
        }
    }
}

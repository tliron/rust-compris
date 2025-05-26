use super::{super::normal::*, error::*, mode::*};

use kutil_std::error::*;

impl Map {
    /// Merge another map into this map. Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_errors<'own, ErrorRecipientT>(
        &mut self,
        other: &'own Self,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own>>
    where
        ErrorRecipientT: ErrorRecipient<MergeError<'own>>,
        Self: 'own,
    {
        let mut changed = false;

        for (other_key, other_value) in &other.value {
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
    ) -> Result<bool, MergeError<'own>> {
        self.merge_with_errors(other, merge_mode, &mut FailFastErrorRecipient)
    }

    /// Merge another map into this map. Return true if any change happened.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool {
        // The default mode should never cause errors, so unwrap is safe
        self.merge_with_mode(other, &MergeMode::default()).expect("merge_with_mode")
    }

    fn merge_key<'own, ErrorRecipientT>(
        &mut self,
        other_key: &'own Value,
        other_value: &'own Value,
        merge_mode: &MergeMode,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, MergeError<'own>>
    where
        ErrorRecipientT: ErrorRecipient<MergeError<'own>>,
    {
        match self.value.get_mut(other_key) {
            Some(value) => {
                // We already have the key, so merge the value
                Ok(value.merge_with_errors(other_value, merge_mode, errors)?)
            }

            None => {
                // We don't have the key, so insert it
                if self.value.insert(other_key.clone(), other_value.clone()).is_some() {
                    if matches!(merge_mode.map, MapMergeMode::FailExisting) {
                        errors.give(MergeError::new(other_key))?;
                    }
                }
                Ok(true)
            }
        }
    }
}

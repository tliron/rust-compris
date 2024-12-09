use super::{list::*, map::*, value::*};

impl Value {
    /// Merge another value into this value. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The list merging behavior depends on the [MergeMode].
    ///
    /// Maps are merged by adding the non-existing key-value pairs and recursively
    /// merging the values of existing keys.
    pub fn merge_from(&mut self, other: &Self, merge_mode: &MergeMode) -> bool {
        match self {
            Self::List(list) => match other {
                Self::List(other_list) => list.merge_from(other_list, merge_mode),
                _ => false,
            },

            Self::Map(map) => match other {
                Self::Map(other_map) => map.merge_from(other_map, merge_mode),
                _ => false,
            },

            _ => false,
        }
    }
}

impl List {
    /// Merge another list into this list. Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_from(&mut self, other: &Self, merge_mode: &MergeMode) -> bool {
        match merge_mode {
            MergeMode::AppendLists => {
                if other.value.is_empty() {
                    false
                } else {
                    self.value.extend(other.value.iter().cloned());
                    true
                }
            }

            MergeMode::AppendListsUnique => {
                let mut changed = false;

                for element in &other.value {
                    if self.push_unique_clone(element) {
                        changed = true;
                    }
                }

                changed
            }

            MergeMode::ReplaceLists => {
                if self == other {
                    false
                } else {
                    self.value = other.value.clone();
                    true
                }
            }
        }
    }
}

impl Map {
    /// Merge another map into this map. Return true if any change happened.
    ///
    /// It will add non-existing key-value pairs and recursively merge the values
    /// of existing keys.
    pub fn merge_from(&mut self, other: &Self, merge_mode: &MergeMode) -> bool {
        let mut changed = false;

        for (key, other_value) in &other.value {
            match self.value.get_mut(key) {
                Some(value) => {
                    // We already have the key, so merge the value
                    if value.merge_from(other_value, merge_mode) {
                        changed = true;
                    }
                }

                None => {
                    // We don't have the key, so insert it
                    self.value.insert(key.clone(), other_value.clone());
                    changed = true;
                }
            }
        }

        changed
    }
}

///
/// MergeMode
///

/// Merge mode.
pub enum MergeMode {
    /// Append lists.
    AppendLists,

    /// Append elements that aren't already contained.
    AppendListsUnique,

    /// Replace lists if they are not equal.
    ReplaceLists,
}

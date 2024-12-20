use super::meta::*;

use std::{fmt::*, hash::*};

//
// Normal
//

/// A normal value.
pub trait Normal: Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Display {
    /// Access to the metadata.
    fn get_meta(&self) -> Option<&Meta>;

    /// Mutable access to the metadata.
    fn get_meta_mut(&mut self) -> Option<&mut Meta>;

    /// Provide a string representation that can be used as a key in a map.
    ///
    /// Useful when non-string keys are not allowed, e.g. for the JSON format.
    fn to_map_string_key(&self) -> String;
}

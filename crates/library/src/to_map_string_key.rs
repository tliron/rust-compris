/// Provide a string representation that can be used as a key in a map.
///
/// Useful when non-string keys are not allowed, e.g. for the JSON format.
pub trait ToMapStringKey {
    /// Returns a string representation that can be used as a key in a map.
    fn to_map_string_key(&self) -> String;
}

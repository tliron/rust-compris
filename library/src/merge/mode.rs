//
// MergeMode
//

/// Merge mode.
#[derive(Debug, Default)]
pub struct MergeMode {
    /// List merge mode.
    pub list: ListMergeMode,

    /// Map merge mode.
    pub map: MapMergeMode,
}

//
// ListMergeMode
//

/// List merge mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ListMergeMode {
    /// Append all items.
    #[default]
    Append,

    /// Skip items that are already contained (treat like a set).
    SkipExisting,

    /// Fail on items that are already contained (treat like a set).
    FailExisting,

    /// Replace lists if they are not equal.
    Replace,
}

//
// MapMergeMode
//

/// Map merge mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MapMergeMode {
    /// Override existing keys.
    #[default]
    OverrideExisting,

    /// Fail on existing keys.
    FailExisting,
}

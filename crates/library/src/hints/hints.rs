//
// Hints
//

/// Hints.
pub struct Hints {
    /// Integer.
    pub integer: String,

    /// Escaped integer.
    pub escaped_integer: String,

    /// Unsigned integer.
    pub unsigned_integer: String,

    /// Escaped unsigned integer.
    pub escaped_unsigned_integer: String,

    /// Bytes.
    pub bytes: String,

    /// Escaped bytes.
    pub escaped_bytes: String,

    /// Map.
    pub map: String,

    /// Escaped map.
    pub escaped_map: String,
}

impl Hints {
    /// Hints for XJSON.
    pub fn xjson() -> Self {
        Self {
            integer: "$hint.int".into(),
            escaped_integer: "$$hint.int".into(),
            unsigned_integer: "$hint.uint".into(),
            escaped_unsigned_integer: "$$hint.uint".into(),
            bytes: "$hint.bytes".into(),
            escaped_bytes: "$$hint.bytes".into(),
            map: "$hint.map".into(),
            escaped_map: "$$hint.map".into(),
        }
    }
}

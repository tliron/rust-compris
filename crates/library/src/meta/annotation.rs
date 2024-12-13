//
// Annotation
//

/// Annotation metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Annotation {
    /// Integer annotation.
    Integer(i64),

    /// String annotation.
    String(String),
}

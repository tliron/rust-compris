//
// Annotation
//

/// Annotation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Annotation {
    /// Integer annotation.
    Integer(i64),

    /// String annotation.
    String(String),
}

//
// AnnotatedDepictionMode
//

/// Mode for [AnnotatedDepiction](super::depiction::AnnotatedDepiction).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotatedDepictionMode {
    /// Inline.
    Inline,

    /// Multiline.
    Multiline,
}

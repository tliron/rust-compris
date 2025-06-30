//
// AnnotatedDebuggableMode
//

/// Mode for [AnnotatedDebuggable](super::debuggable::AnnotatedDebuggable).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotatedDebuggableMode {
    /// Inline.
    Inline,

    /// Multiline.
    Multiline,
}

use owo_colors::*;

/// Styles used for pretty textual serialization, including [WriteDebug](super::WriteDebug).
pub struct Styles {
    /// For plain text.
    pub plain: Style,

    /// For numbers.
    pub number: Style,

    /// For strings.
    pub string: Style,

    /// For metadata.
    pub meta: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            plain: Style::new().yellow(),
            number: Style::new().magenta(),
            string: Style::new().blue(),
            meta: Style::new().green(),
        }
    }
}

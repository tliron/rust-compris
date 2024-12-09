use owo_colors::*;

/// Styles used for pretty textual serialization, including [WriteDebug](super::WriteDebug).
pub struct Styles {
    pub plain: Style,
    pub number: Style,
    pub string: Style,
    pub location: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            plain: Style::new().yellow(),
            number: Style::new().magenta(),
            string: Style::new().blue(),
            location: Style::new().green(),
        }
    }
}

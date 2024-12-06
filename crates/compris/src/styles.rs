use owo_colors::*;

pub struct Styles {
    pub plain: Style,
    pub number: Style,
    pub string: Style,
    pub location: Style,
}

impl Styles {
    pub fn new() -> Self {
        Self {
            plain: Style::new().yellow(),
            number: Style::new().magenta(),
            string: Style::new().blue(),
            location: Style::new().green(),
        }
    }
}

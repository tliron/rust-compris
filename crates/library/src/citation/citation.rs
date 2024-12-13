use super::super::normal::*;

//
// Citation
//

/// How to find me!
#[derive(Debug, Clone, Default)]
pub struct Citation {
    /// Optional source.
    pub source: Option<String>,

    /// Optional coordinates.
    pub coordinates: Option<Coordinates>,

    /// Optional path.
    pub path: Option<String>,
}

impl Citation {
    /// Constructor.
    pub fn new(source: Option<String>, coordinates: Option<Coordinates>, path: Option<String>) -> Self {
        Self { source, coordinates, path }
    }
}

//
// HasCitation
//

/// Has [Citation].
pub trait HasCitation: Sized {
    /// Gets the citation.
    fn get_citation(&self) -> &Citation;

    /// Sets the citation.
    fn with_citation(self, citation: Citation) -> Self;
}

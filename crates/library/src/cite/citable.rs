use super::citation::*;

//
// Citable
//

/// Has a [Citation].
pub trait Citable: Sized {
    /// Gets the citation.
    fn get_citation(&self) -> Option<&Citation>;

    /// Gets the citation.
    fn get_citation_mut(&mut self) -> Option<&mut Citation>;

    /// Sets the citation.
    fn with_citation(mut self, citation: Citation) -> Self {
        if let Some(our_citation) = self.get_citation_mut() {
            *our_citation = citation;
        }
        self
    }

    /// Sets the citation.
    fn with_citation_option(self, citation: Option<&Citation>) -> Self {
        match citation {
            Some(citation) => self.with_citation(citation.clone()),
            None => self,
        }
    }

    /// Sets the citation from another citable.
    fn with_citation_from<CitableT>(self, citable: &CitableT) -> Self
    where
        CitableT: Citable,
    {
        self.with_citation_option(citable.get_citation())
    }
}

//
// CitableFields
//

/// Has a [Citation] for its fields.
pub trait CitableFields: Citable {
    /// Get a field citation.
    fn get_field_citation(&self, name: &str) -> Option<&Citation>;

    /// Get a field citation, or if there is none get our own citation.
    fn get_field_or_own_citation(&self, name: &str) -> Option<&Citation> {
        match self.get_field_citation(name) {
            Some(citation) => Some(citation),
            None => match self.get_citation() {
                Some(citation) => Some(citation),
                None => None,
            },
        }
    }

    /// Get a field citation, or if there is none clone our own citation and add the field to the
    /// path.
    fn get_field_citation_always(&self, name: &str) -> Option<Citation> {
        match self.get_field_citation(name) {
            Some(citation) => Some(citation.clone()),
            None => match self.get_citation() {
                Some(citation) => Some(citation.with_map_key(name.into())),
                None => None,
            },
        }
    }
}

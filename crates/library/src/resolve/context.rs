use super::super::{citation::*, normal::*};

//
// ResolveContext
//

/// Resolve context.
#[derive(Debug, Clone, Default)]
pub struct ResolveContext<'a> {
    /// The optional source.
    pub source: Option<&'a String>,
}

impl<'a> ResolveContext<'a> {
    /// Constructor.
    pub fn new(source: Option<&'a String>) -> Self {
        Self { source }
    }
}

//
// Citation
//

impl Citation {
    /// Constructor.
    pub fn new_for(value: &Value, ancestor: Option<&Value>, context: Option<&ResolveContext>) -> Self {
        let (source, coordinates, path) = (
            match context {
                Some(context) => match context.source {
                    Some(source) => Some(source.clone()),
                    None => None,
                },
                None => None,
            },
            match value.get_meta() {
                Some(meta) => meta.coordinates.clone(),
                None => None,
            },
            match ancestor {
                Some(ancestor) => match Path::find(ancestor, value) {
                    Some(path) => {
                        if path.is_linear() {
                            Some(path.to_string())
                        } else {
                            None
                        }
                    }
                    None => None,
                },
                None => None,
            },
        );

        Self::new(source, coordinates, path)
    }
}

//
// WithCitationFor
//

/// Sets the citation.
pub trait WithCitationFor {
    /// Sets the citation.
    fn with_citation_for(self, value: &Value, ancestor: Option<&Value>, context: Option<&ResolveContext>) -> Self;
}

impl<T: HasCitation> WithCitationFor for T {
    fn with_citation_for(self, value: &Value, ancestor: Option<&Value>, context: Option<&ResolveContext>) -> Self {
        self.with_citation(Citation::new_for(value, ancestor, context))
    }
}

use super::super::context::*;

//
// CommonResolveContext
//

/// Common resolve context.
#[derive(Debug, Clone, Default)]
pub struct CommonResolveContext {
    /// The optional source.
    pub source: Option<String>,
}

impl CommonResolveContext {
    /// Constructor.
    pub fn new(source: Option<String>) -> Self {
        Self { source }
    }
}

impl ResolveContext for CommonResolveContext {
    fn get_source(&self) -> Option<&String> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

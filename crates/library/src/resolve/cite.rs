use super::{
    super::{cite::*, meta::*, normal::*, path::*},
    context::*,
};

//
// Citation
//

impl Citation {
    /// Constructor.
    pub fn new_for<ContextT>(value: &Value, context: Option<&ContextT>, ancestor: Option<&Value>) -> Self
    where
        ContextT: ResolveContext,
    {
        let (source, location, path) = (
            context.and_then(|context| context.get_source().map(|source| source.clone())),
            value.get_meta().and_then(|meta| meta.location.clone()),
            ancestor.and_then(|ancestor| Path::find(ancestor, value)),
        );

        Self::new(source, location, path)
    }
}

//
// WithCitationFor
//

/// Sets the [Citation].
pub trait WithCitationFor {
    /// Sets the citation.
    fn with_citation_for<ResolveContextT>(
        self,
        value: &Value,
        context: Option<&ResolveContextT>,
        ancestor: Option<&Value>,
    ) -> Self
    where
        ResolveContextT: ResolveContext;
}

impl<CitableT> WithCitationFor for CitableT
where
    CitableT: Citable,
{
    fn with_citation_for<ResolveContextT>(
        self,
        value: &Value,
        context: Option<&ResolveContextT>,
        ancestor: Option<&Value>,
    ) -> Self
    where
        ResolveContextT: ResolveContext,
    {
        self.with_citation(Citation::new_for(value, context, ancestor))
    }
}

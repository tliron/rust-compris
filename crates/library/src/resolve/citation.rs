use super::{
    super::{citation::*, normal::*},
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
        let (source, coordinates, path) = (
            match context {
                Some(context) => match context.get_source() {
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
// CitableFields
//

/// Has a [Citation] for its fields.
pub trait CitableFields {
    /// Get a field citation.
    fn get_field_citation(&self, name: &str) -> Option<&Citation>;
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

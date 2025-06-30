use super::super::{
    super::{annotation::*, normal::*},
    errors::*,
    resolve::*,
};

use kutil_std::error::*;

// Resolving a value into a value means cloning it

impl<ResolvedAnnotationsT, AnnotatedT> Resolve<Value<ResolvedAnnotationsT>, AnnotatedT> for Value<AnnotatedT>
where
    ResolvedAnnotationsT: Annotated + Default,
    AnnotatedT: Annotated + Clone,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        _errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Value<ResolvedAnnotationsT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(Some(self.clone().into_annotated()))
    }
}

use super::super::{
    super::{annotation::*, normal::*},
    errors::*,
    resolve::*,
};

use kutil_std::error::*;

// Resolving a value into a value means cloning it

impl<ResolvedAnnotationsT, AnnotationsT> Resolve<Value<ResolvedAnnotationsT>, AnnotationsT> for Value<AnnotationsT>
where
    ResolvedAnnotationsT: Annotated + Default,
    AnnotationsT: Annotated + Clone,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        _errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Value<ResolvedAnnotationsT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        Ok(Some(self.clone().into_annotated()))
    }
}

use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use kutil::std::error::*;

// Resolving a variant into a variant means cloning it

impl<ResolvedAnnotationsT, AnnotatedT> Resolve<Variant<ResolvedAnnotationsT>, AnnotatedT> for Variant<AnnotatedT>
where
    ResolvedAnnotationsT: Annotated + Default,
    AnnotatedT: Annotated + Clone,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        _errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Variant<ResolvedAnnotationsT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(Some(self.clone().into_annotated()))
    }
}

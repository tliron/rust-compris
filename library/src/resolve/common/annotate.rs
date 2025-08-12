use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use kutil::std::error::*;

impl<InnerT, AnnotatedT> Resolve<Annotate<InnerT, AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Default,
    Variant<AnnotatedT>: Resolve<InnerT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Annotate<InnerT, AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(self.resolve_with_errors(errors)?.map(|inner| Annotate::new(inner).with_annotations_from(self)))
    }
}

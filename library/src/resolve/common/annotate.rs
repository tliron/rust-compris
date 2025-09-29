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
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Annotate<InnerT, AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let annotations = self.annotations().cloned();
        Ok(self.resolve_with_errors(errors)?.map(|inner| {
            let annotate = Annotate::new(inner);
            match annotations {
                Some(annotations) => annotate.with_annotations(annotations),
                None => annotate,
            }
        }))
    }
}

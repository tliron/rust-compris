use super::super::{
    super::{
        super::{annotation::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use kutil_std::error::*;

impl<ItemT, AnnotatedT> Resolve<Vec<ItemT>, AnnotatedT> for Value<AnnotatedT>
where
    Value<AnnotatedT>: Resolve<ItemT, AnnotatedT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Vec<ItemT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved = Vec::new();

        if let Some(mut iterator) = ResolvingValueIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push(item);
            }
        }

        Ok(Some(resolved))
    }
}

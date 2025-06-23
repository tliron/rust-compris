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

impl<ItemT, AnnotationsT> Resolve<Vec<ItemT>, AnnotationsT> for Value<AnnotationsT>
where
    Value<AnnotationsT>: Resolve<ItemT, AnnotationsT>,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Vec<ItemT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
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

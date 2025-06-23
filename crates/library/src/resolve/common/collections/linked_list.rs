use super::super::{
    super::{
        super::{annotation::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {kutil_std::error::*, std::collections::*};

// Uses push_back

impl<ItemT, AnnotationsT> Resolve<LinkedList<ItemT>, AnnotationsT> for Value<AnnotationsT>
where
    Value<AnnotationsT>: Resolve<ItemT, AnnotationsT>,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<LinkedList<ItemT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        let mut resolved = LinkedList::new();

        if let Some(mut iterator) = ResolvingValueIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push_back(item);
            }
        }

        Ok(Some(resolved))
    }
}

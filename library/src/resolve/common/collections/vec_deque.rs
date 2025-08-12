use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {kutil::std::error::*, std::collections::*};

// Uses push_back

impl<ItemT, AnnotatedT> Resolve<VecDeque<ItemT>, AnnotatedT> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ItemT, AnnotatedT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<VecDeque<ItemT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved = VecDeque::default();

        if let Some(mut iterator) = ResolvingVariantIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push_back(item);
            }
        }

        Ok(Some(resolved))
    }
}

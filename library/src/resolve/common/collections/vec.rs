use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use kutil::std::error::*;

impl<ItemT, AnnotatedT> Resolve<Vec<ItemT>, AnnotatedT> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ItemT, AnnotatedT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Vec<ItemT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved = Vec::default();

        if let Some(mut iterator) = ResolvingVariantIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push(item);
            }
        }

        Ok(Some(resolved))
    }
}

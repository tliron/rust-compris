use super::super::{
    super::{super::normal::*, context::*, error::*, iterator::*, resolve::*, result::*},
    iterate::*,
};

use kutil_std::error::*;

impl<ItemT, ContextT, ErrorT> Resolve<Vec<ItemT>, ContextT, ErrorT> for Value
where
    Value: Resolve<ItemT, ContextT, ErrorT>,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<'own, ErrorRecipientT>(
        &'own self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'own Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Vec<ItemT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut resolved = Vec::new();

        if let Some(mut iterator) = ResolvingValueIterator::new_from(self, context, ancestor, errors)? {
            while let Some(item) = iterator.resolve_next(context, ancestor, errors)? {
                resolved.push(item);
            }
        }

        Ok(Some(resolved))
    }
}

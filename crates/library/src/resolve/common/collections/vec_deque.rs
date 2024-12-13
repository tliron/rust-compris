use super::super::{
    super::{super::normal::*, context::*, error::*, iterator::*, resolve::*, result::*},
    iterate::*,
};

use {kutil_std::error::*, std::collections::*};

// Uses push_back

impl<ItemT, ContextT, ErrorT> Resolve<VecDeque<ItemT>, ContextT, ErrorT> for Value
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
    ) -> ResolveResult<VecDeque<ItemT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut resolved = VecDeque::new();

        if let Some(mut iterator) = ResolvingValueIterator::new_from(self, context, ancestor, errors)? {
            while let Some(item) = iterator.resolve_next(context, ancestor, errors)? {
                resolved.push_back(item);
            }
        }

        Ok(Some(resolved))
    }
}

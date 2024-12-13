use super::super::{
    super::{super::normal::*, context::*, error::*, iterator::*, resolve::*, result::*},
    iterate::*,
};

use {
    kutil_std::error::*,
    std::{collections::*, hash::*},
};

impl<KeyT, ValueT, ContextT, ErrorT> Resolve<BTreeMap<KeyT, ValueT>, ContextT, ErrorT> for Value
where
    KeyT: Hash + Eq + Ord,
    Value: Resolve<KeyT, ContextT, ErrorT> + Resolve<ValueT, ContextT, ErrorT>,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<'own, ErrorRecipientT>(
        &'own self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'own Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<BTreeMap<KeyT, ValueT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        let mut resolved = BTreeMap::new();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, context, ancestor, errors)? {
            while let Some((key, value)) = iterator.resolve_next(context, ancestor, errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}

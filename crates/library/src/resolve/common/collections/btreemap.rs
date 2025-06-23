use super::super::{
    super::{
        super::{annotation::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {
    kutil_std::error::*,
    std::{collections::*, hash::*},
};

impl<KeyT, ValueT, AnnotationsT> Resolve<BTreeMap<KeyT, ValueT>, AnnotationsT> for Value<AnnotationsT>
where
    KeyT: Hash + Eq + Ord,
    Value<AnnotationsT>: Resolve<KeyT, AnnotationsT> + Resolve<ValueT, AnnotationsT>,
    AnnotationsT: Annotated + Clone + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<BTreeMap<KeyT, ValueT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        let mut resolved = BTreeMap::new();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, errors)? {
            while let Some((key, value)) = iterator.resolve_next(errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}

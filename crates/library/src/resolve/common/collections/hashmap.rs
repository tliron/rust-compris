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

impl<KeyT, ValueT, BuildHasherT, AnnotationsT> Resolve<HashMap<KeyT, ValueT, BuildHasherT>, AnnotationsT>
    for Value<AnnotationsT>
where
    KeyT: Hash + Eq,
    Value<AnnotationsT>: Resolve<KeyT, AnnotationsT> + Resolve<ValueT, AnnotationsT>,
    AnnotationsT: Annotated + Clone + Default,
    BuildHasherT: BuildHasher + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<HashMap<KeyT, ValueT, BuildHasherT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        let mut resolved = HashMap::default();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, errors)? {
            while let Some((key, value)) = iterator.resolve_next(errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}

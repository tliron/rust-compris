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

impl<KeyT, ValueT, BuildHasherT, AnnotatedT> Resolve<HashMap<KeyT, ValueT, BuildHasherT>, AnnotatedT>
    for Value<AnnotatedT>
where
    KeyT: Hash + Eq,
    Value<AnnotatedT>: Resolve<KeyT, AnnotatedT> + Resolve<ValueT, AnnotatedT>,
    AnnotatedT: Annotated + Clone + Default,
    BuildHasherT: BuildHasher + Default,
{
    fn resolve_with_errors<'own, ErrorRecipientT>(
        &'own self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<HashMap<KeyT, ValueT, BuildHasherT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
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

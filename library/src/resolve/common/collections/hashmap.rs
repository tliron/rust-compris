use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {
    kutil::std::error::*,
    std::{collections::*, hash::*},
};

impl<KeyT, ValueT, BuildHasherT, AnnotatedT> Resolve<HashMap<KeyT, ValueT, BuildHasherT>, AnnotatedT>
    for Variant<AnnotatedT>
where
    KeyT: Hash + Eq,
    Variant<AnnotatedT>: Resolve<KeyT, AnnotatedT> + Resolve<ValueT, AnnotatedT>,
    AnnotatedT: 'static + Annotated + Clone + Default,
    BuildHasherT: BuildHasher + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
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

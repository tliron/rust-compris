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

impl<KeyT, ValueT, AnnotatedT> Resolve<BTreeMap<KeyT, ValueT>, AnnotatedT> for Variant<AnnotatedT>
where
    KeyT: Hash + Eq + Ord,
    Variant<AnnotatedT>: Resolve<KeyT, AnnotatedT> + Resolve<ValueT, AnnotatedT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<BTreeMap<KeyT, ValueT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved = BTreeMap::default();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, errors)? {
            while let Some((key, value)) = iterator.resolve_next(errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}

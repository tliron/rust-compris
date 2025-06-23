use super::super::{super::normal::*, errors::*, resolve::*};

use kutil_std::error::*;

// We only have to care about Some, because None will never get resolved
// (A Null is definitely not a None and requires entirely different consideration)

impl<OptionalT, AnnotationsT> Resolve<Option<OptionalT>, AnnotationsT> for Value<AnnotationsT>
where
    Value<AnnotationsT>: Resolve<OptionalT, AnnotationsT>,
{
    fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) -> ResolveResult<Option<OptionalT>, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        Ok(Some(self.resolve_with_errors(errors)?))
    }
}

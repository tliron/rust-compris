use super::super::{super::normal::*, resolve::*, result::*};

use kutil_std::error::*;

// We only have to care about Some, because None will never get resolved
// (A Null is definitely not a None and requires entirely different consideration)

impl<OptionalT, ContextT, ErrorT> Resolve<Option<OptionalT>, ContextT, ErrorT> for Value
where
    Value: Resolve<OptionalT, ContextT, ErrorT>,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Option<OptionalT>, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(Some(self.resolve_for(context, ancestor, errors)?))
    }
}

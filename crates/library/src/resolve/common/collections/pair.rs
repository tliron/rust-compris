use super::super::super::{super::normal::*, resolve::*, result::*};

use kutil_std::error::*;

// Resolve two values at once
// Useful for key-value pairs of maps

impl<FirstT, SecondT, ContextT, ErrorT> Resolve<(FirstT, SecondT), ContextT, ErrorT> for (&Value, &Value)
where
    Value: Resolve<FirstT, ContextT, ErrorT> + Resolve<SecondT, ContextT, ErrorT>,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<(FirstT, SecondT), ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        let first = self.0.resolve_for(context, ancestor, errors)?;
        let second = self.1.resolve_for(context, ancestor, errors)?;

        if let Some(first) = first {
            if let Some(second) = second {
                return Ok(Some((first, second)));
            }
        }

        Ok(None)
    }
}

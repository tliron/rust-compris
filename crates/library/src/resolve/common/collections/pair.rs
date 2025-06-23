use super::super::super::{super::normal::*, errors::*, resolve::*};

use kutil_std::error::*;

// Resolve two values at once
// Useful for key-value pairs of maps

impl<FirstT, SecondT, AnnotationsT> Resolve<(FirstT, SecondT), AnnotationsT>
    for (&Value<AnnotationsT>, &Value<AnnotationsT>)
where
    Value<AnnotationsT>: Resolve<FirstT, AnnotationsT> + Resolve<SecondT, AnnotationsT>,
{
    fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) -> ResolveResult<(FirstT, SecondT), AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
    {
        let first = self.0.resolve_with_errors(errors)?;
        let second = self.1.resolve_with_errors(errors)?;

        Ok(
            if let Some(first) = first
                && let Some(second) = second
            {
                Some((first, second))
            } else {
                None
            },
        )
    }
}

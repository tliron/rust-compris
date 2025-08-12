use super::super::super::{super::normal::*, errors::*, resolve::*};

use kutil::std::error::*;

// Resolve two values at once
// Useful for key-value pairs of maps

impl<FirstT, SecondT, AnnotatedT> Resolve<(FirstT, SecondT), AnnotatedT>
    for (&Variant<AnnotatedT>, &Variant<AnnotatedT>)
where
    Variant<AnnotatedT>: Resolve<FirstT, AnnotatedT> + Resolve<SecondT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<(FirstT, SecondT), AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
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

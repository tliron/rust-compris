use super::{super::normal::*, context::*, error::*, result::*};

use kutil_std::error::*;

/// Iterator that resolves one item at a time.
pub trait ResolvingIterator<ResolvedT, ContextT, ErrorT>
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    /// Resolve next.
    ///
    /// Important: An error returned here does *not* mean that there are no more entries,
    /// just that the current iteration caused an error. Future ones might not. To exhaust
    /// the iterator, keep calling this function until it returns [None].
    fn resolve_next<ErrorRecipientT>(
        &mut self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>;
}

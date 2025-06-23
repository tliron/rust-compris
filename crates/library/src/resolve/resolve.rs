use super::errors::*;

use kutil_std::error::*;

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<ResolvedT, AnnotationsT> {
    /// Resolve one type into another.
    ///
    /// Errors can be reported as usual by [Err] *but also* by the [ErrorRecipient]. Callers should
    /// thus check that `errors` is empty even when the function returns [Ok].
    ///
    /// The function may return [Some] partially resolved result even if there are errors.
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>;

    /// Resolve one type into another.
    ///
    /// Unlike [resolve](Resolve::resolve) will fail on the first encountered error and will return
    /// [ResolveError::None] instead of [None].
    ///
    /// If you want all the errors use [resolve](Resolve::resolve) instead.
    fn resolve(&self) -> Result<ResolvedT, ResolveError<AnnotationsT>> {
        self.resolve_with_errors(&mut FailFastErrorRecipient)?.ok_or(ResolveError::None)
    }
}

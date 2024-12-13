use super::{super::normal::*, context::*, resolve_error::*, result::*};

use kutil_std::error::*;

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<T, E: ResolveError> {
    /// Resolve one type into another.
    ///
    /// In the case of an error, implementations should report errors using [ErrorRecipient::give] on
    /// the provided errors.
    ///
    /// Only if [give](ErrorRecipient::give) itself fails should they return an error here, which allows
    /// for a fail-fast mode. They should not otherwise return an error.
    ///
    /// The optional ancestor and context arguments can be used both for resolution and for adding more
    /// details to errors.
    fn resolve_for<ER: ErrorRecipient<E>>(
        &self,
        ancestor: Option<&Value>,
        context: Option<&ResolveContext>,
        errors: &mut ER,
    ) -> ResolveResult<T, E>;

    /// Resolve one type into another.
    fn resolve<ER: ErrorRecipient<E>>(&self, errors: &mut ER) -> ResolveResult<T, E> {
        self.resolve_for(None, None, errors)
    }

    /// Resolve one type into another while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    fn resolve_fail_fast(&self) -> ResolveResult<T, E> {
        self.resolve(&mut FailFastErrorRecipient)
    }
}

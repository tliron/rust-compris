use super::{super::normal::*, common::*, result::*};

use kutil_std::error::*;

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<ResolvedT, ContextT = CommonResolveContext, ErrorT = CommonResolveError> {
    /// Resolve one type into another.
    ///
    /// In the case of an error, implementations should report errors using [ErrorRecipient::give].
    ///
    /// Only if [give](ErrorRecipient::give) itself fails should they return an error here, which allows
    /// for a fail-fast mode. They should not otherwise return an error.
    ///
    /// The optional context and ancestor arguments can be used both for resolution and for adding more
    /// details to errors.
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>;

    /// Resolve one type into another with a provided [ErrorRecipient].
    fn resolve_into<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) -> ResolveResult<ResolvedT, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        self.resolve_for(None, None, errors)
    }

    /// Resolve one type into another while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    fn resolve(&self) -> ResolveResult<ResolvedT, ErrorT> {
        self.resolve_into(&mut FailFastErrorRecipient)
    }
}

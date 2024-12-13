use super::{
    super::{super::normal::*, context::*, error::*, resolve::*, result::*},
    from_str::*,
};

use {kutil_std::error::*, std::path::*};

impl<ContextT, ErrorT> Resolve<PathBuf, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<PathBuf, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        resolve_from_str(self, context, ancestor, errors)
    }
}

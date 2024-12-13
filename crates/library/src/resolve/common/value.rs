use super::super::{super::normal::*, resolve::*, result::*};

use kutil_std::error::*;

// Resolving a value into a value means cloning it

impl<ContextT, ErrorT> Resolve<Value, ContextT, ErrorT> for Value {
    fn resolve_for<ErrorRecipientT>(
        &self,
        _context: Option<&ContextT>,
        _ancestor: Option<&Value>,
        _errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Value, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(Some(self.clone()))
    }
}

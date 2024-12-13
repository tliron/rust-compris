use super::{
    super::{normal::*, read::*},
    context::*,
    error::*,
    resolve::*,
    result::*,
};

use {kutil_std::error::*, std::io};

//
// Reader
//

impl Reader {
    /// Resolve the read value into another type.
    pub fn resolve<ResolvedT, ReadT, ContextT, ErrorT, ErrorRecipientT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ReadT: io::Read,
        ContextT: ResolveContext,
        ErrorT: ResolveError,
        ErrorRecipientT: ErrorRecipient<ErrorT>,
        Value: Resolve<ResolvedT, ContextT, ErrorT>,
    {
        let value = self.read(reader).unwrap();
        value.resolve_into(errors)
    }

    /// Resolve the read value into another type.
    pub fn resolve_from_string<ResolvedT, ContextT, ErrorT, ErrorRecipientT>(
        &self,
        string: &str,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ContextT: ResolveContext,
        ErrorT: ResolveError,
        ErrorRecipientT: ErrorRecipient<ErrorT>,
        Value: Resolve<ResolvedT, ContextT, ErrorT>,
    {
        let value = self.read_from_string(string).unwrap();
        value.resolve_into(errors)
    }

    /// Resolve the read value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_fail_fast<ResolvedT, ReadT, ContextT, ErrorT>(
        &self,
        reader: &mut ReadT,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ReadT: io::Read,
        ContextT: ResolveContext,
        ErrorT: ResolveError,
        Value: Resolve<ResolvedT, ContextT, ErrorT>,
    {
        self.resolve(reader, &mut FailFastErrorRecipient)
    }

    /// Resolve the read value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_from_string_fail_fast<ResolvedT, ContextT, ErrorT>(
        &self,
        string: &str,
    ) -> ResolveResult<ResolvedT, ErrorT>
    where
        ContextT: ResolveContext,
        ErrorT: ResolveError,
        Value: Resolve<ResolvedT, ContextT, ErrorT>,
    {
        self.resolve_from_string(string, &mut FailFastErrorRecipient)
    }
}

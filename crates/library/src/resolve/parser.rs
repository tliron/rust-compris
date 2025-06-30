use super::{
    super::{annotation::*, normal::*, parse::*},
    errors::*,
    resolve::*,
};

use {kutil_std::error::*, std::io};

impl Parser {
    /// Resolve the parsed value into another type.
    pub fn resolve<ResolvedT, ReadT, AnnotatedT, ErrorRecipientT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
        Value<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let value = self.parse(reader).expect("parse");
        value.resolve_with_errors(errors)
    }

    /// Resolve the parsed value into another type.
    pub fn resolve_from_string<ResolvedT, AnnotatedT, ErrorRecipientT>(
        &self,
        string: &str,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
        Value<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let value = self.parse_from_string(string).expect("parse");
        value.resolve_with_errors(errors)
    }

    /// Resolve the parsed value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_fail_fast<ResolvedT, ReadT, AnnotatedT>(
        &self,
        reader: &mut ReadT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        Value<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve(reader, &mut FailFastErrorRecipient)
    }

    /// Resolve the parsed value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_from_string_fail_fast<ResolvedT, AnnotatedT>(
        &self,
        string: &str,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        Value<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve_from_string(string, &mut FailFastErrorRecipient)
    }
}

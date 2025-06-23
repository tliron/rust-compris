use super::{
    super::{annotation::*, normal::*, parse::*},
    errors::*,
    resolve::*,
};

use {kutil_std::error::*, std::io};

impl Parser {
    /// Resolve the parsed value into another type.
    pub fn resolve<ResolvedT, ReadT, AnnotationsT, ErrorRecipientT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
        Value<AnnotationsT>: Resolve<ResolvedT, AnnotationsT>,
    {
        let value = self.parse(reader).expect("parse");
        value.resolve_with_errors(errors)
    }

    /// Resolve the parsed value into another type.
    pub fn resolve_from_string<ResolvedT, AnnotationsT, ErrorRecipientT>(
        &self,
        string: &str,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        AnnotationsT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotationsT>>,
        Value<AnnotationsT>: Resolve<ResolvedT, AnnotationsT>,
    {
        let value = self.parse_from_string(string).expect("parse");
        value.resolve_with_errors(errors)
    }

    /// Resolve the parsed value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_fail_fast<ResolvedT, ReadT, AnnotationsT>(
        &self,
        reader: &mut ReadT,
    ) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
        Value<AnnotationsT>: Resolve<ResolvedT, AnnotationsT>,
    {
        self.resolve(reader, &mut FailFastErrorRecipient)
    }

    /// Resolve the parsed value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_from_string_fail_fast<ResolvedT, AnnotationsT>(
        &self,
        string: &str,
    ) -> ResolveResult<ResolvedT, AnnotationsT>
    where
        AnnotationsT: Annotated + Clone + Default,
        Value<AnnotationsT>: Resolve<ResolvedT, AnnotationsT>,
    {
        self.resolve_from_string(string, &mut FailFastErrorRecipient)
    }
}

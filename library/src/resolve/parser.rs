use super::{
    super::{annotate::*, normal::*, parse::*},
    errors::*,
    resolve::*,
};

use {kutil::std::error::*, std::io};

impl Parser {
    /// Resolve the parsed [Variant] into another type.
    pub fn resolve<ResolvedT, ReadT, AnnotatedT, ErrorRecipientT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let variant = self.parse(reader).expect("parse");
        variant.resolve_with_errors(errors)
    }

    /// Resolve the parsed [Variant] into another type.
    pub fn resolve_from_string<ResolvedT, AnnotatedT, ErrorRecipientT>(
        &self,
        string: &str,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let variant = self.parse_from_string(string).expect("parse");
        variant.resolve_with_errors(errors)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_fail_fast<ResolvedT, ReadT, AnnotatedT>(
        &self,
        reader: &mut ReadT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve(reader, &mut FailFastErrorRecipient)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_from_string_fail_fast<ResolvedT, AnnotatedT>(
        &self,
        string: &str,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve_from_string(string, &mut FailFastErrorRecipient)
    }
}

use super::{
    super::{normal::*, read::*},
    resolve::*,
    resolve_error::*,
    result::*,
};

use {kutil_std::error::*, std::io};

//
// Reader
//

impl Reader {
    /// Resolve the read value into another type.
    pub fn resolve<T, R: io::Read, E: ResolveError, ER: ErrorRecipient<E>>(
        &self,
        reader: &mut R,
        errors: &mut ER,
    ) -> ResolveResult<T, E>
    where
        Value: Resolve<T, E>,
    {
        let value = self.read(reader).unwrap();
        value.resolve(errors)
    }

    /// Resolve the read value into another type.
    pub fn resolve_from_string<T, E: ResolveError, ER: ErrorRecipient<E>>(
        &self,
        string: &str,
        errors: &mut ER,
    ) -> ResolveResult<T, E>
    where
        Value: Resolve<T, E>,
    {
        let value = self.read_from_string(string).unwrap();
        value.resolve(errors)
    }

    /// Resolve the read value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_fail_fast<T, R: io::Read, E: ResolveError>(&self, reader: &mut R) -> ResolveResult<T, E>
    where
        Value: Resolve<T, E>,
    {
        self.resolve(reader, &mut FailFastErrorRecipient)
    }

    /// Resolve the read value into another type while failing on the first encountered error.
    ///
    /// Uses [FailFastErrorRecipient].
    pub fn resolve_from_string_fail_fast<T, E: ResolveError>(&self, string: &str) -> ResolveResult<T, E>
    where
        Value: Resolve<T, E>,
    {
        self.resolve_from_string(string, &mut FailFastErrorRecipient)
    }
}

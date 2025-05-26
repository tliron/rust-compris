use super::super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*};

use {bytes::*, bytestring::*, duplicate::*, kutil_std::error::*};

// Note that Strings will be cloned, so using ByteString is more efficient

#[duplicate_item(
  _Resolved;
  [isize];
  [i64];
  [i32];
  [i16];
  [i8];
  [usize];
  [u64];
  [u32];
  [u16];
  [u8];
  [f64];
  [f32];
  [bool];
  [ByteString];
  [String];
  [Bytes];
)]
impl<ContextT, ErrorT> Resolve<_Resolved, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<_Resolved, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self.try_into() {
            Ok(primitive) => Some(primitive),

            Err(error) => {
                errors.give(error.with_citation_for(self, context, ancestor))?;
                None
            }
        })
    }
}

// Failed attempt at blanket generic:
//
// impl<'own, ResolvedT, ContextT, ErrorT> Resolve<ResolvedT, ContextT, ErrorT> for Value
// where
//     ContextT: ResolveContext,
//     ErrorT: ResolveError,
//     &'own Self: TryInto<ResolvedT, Error = IncompatibleValueTypeError>,
// {
//     fn resolve_for<ErrorRecipientT>(
//         &self,
//         _context: Option<&ContextT>,
//         _ancestor: Option<&Value>,
//         _errors: &mut ErrorRecipientT,
//     ) -> ResolveResult<ResolvedT, ErrorT>
//     where
//         ErrorRecipientT: ErrorRecipient<ErrorT>,
//     {
//         Ok(match self.try_into() { // ouch, lifetimes!!!!!!!
//             Ok(resolved) => Some(resolved),
//             Err(_err) => {
//                 //_errors.give(_err.with_citation_for(self, _context, _ancestor))?;
//                 None
//             }
//         })
//     }
// }

use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    duplicate::*,
    kutil_std::{error::*, zerocopy::*},
};

// Note that Strings will be cloned, so using ByteString is more efficient

#[duplicate_item(
  ResolvedT;
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
impl<AnnotatedT> Resolve<ResolvedT, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(match self.try_into() {
            Ok(native) => Some(native),

            Err(error) => {
                errors.give(error.with_annotations_from(self))?;
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

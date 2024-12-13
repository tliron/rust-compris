use super::super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*};

use kutil_std::error::*;

//
// VecU8
//

/// Trivial [Vec]\<u8\> newtype wrapper that can be used as a struct field for resolving
/// [Bytes].
///
/// The reason we can't use [Vec]\<u8\> directly is because we treat [Vec] as a collection
/// target, meaning we would try to resolve each u8, one at a time. That's not what we want
/// for bytes.
///
/// There are many libraries providing powerful byte buffers types for various purposes.
/// It is easy enough to create a custom [Resolve] implementation for any of those or
/// your own custom type if preferred to this trivial wrapper.
#[derive(Debug, Default)]
pub struct VecU8(pub Vec<u8>);

impl From<VecU8> for Vec<u8> {
    fn from(value: VecU8) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for VecU8 {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl<ContextT, ErrorT> Resolve<VecU8, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<VecU8, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self {
            Self::Bytes(bytes) => Some(VecU8(bytes.value.clone())),

            Self::Text(text) => Some(VecU8(text.value.clone().into())),

            _ => {
                errors.give(
                    IncompatibleValueTypeError::new(self, &["bytes"]).with_citation_for(self, context, ancestor),
                )?;
                None
            }
        })
    }
}

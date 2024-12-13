use super::super::{super::normal::*, cite::*, context::*, error::*, resolve::*, result::*};

use kutil_std::error::*;

//
// Blob
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
#[derive(Clone, Debug, Default)]
pub struct Blob(pub Vec<u8>);

impl From<Vec<u8>> for Blob {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Blob> for Vec<u8> {
    fn from(value: Blob) -> Self {
        value.0
    }
}

impl<'own> From<&'own Blob> for &'own Vec<u8> {
    fn from(value: &'own Blob) -> Self {
        &value.0
    }
}

impl<ContextT, ErrorT> Resolve<Blob, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<ErrorRecipientT>(
        &self,
        context: Option<&ContextT>,
        ancestor: Option<&Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Blob, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        Ok(match self {
            Self::Bytes(bytes) => Some(Blob(bytes.value.clone())),

            Self::Text(text) => Some(Blob(text.value.clone().into())),

            _ => {
                errors.give(
                    IncompatibleValueTypeError::new(self, &["bytes"]).with_citation_for(self, context, ancestor),
                )?;
                None
            }
        })
    }
}

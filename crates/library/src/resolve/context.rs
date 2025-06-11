use bytestring::*;

//
// ResolveContext
//

/// Resolve context.
///
/// See source code for [CommonResolveContext](super::common::CommonResolveContext) for an
/// example of how to implement your own.
pub trait ResolveContext {
    /// The optional source.
    fn get_source(&self) -> Option<&ByteString>;
}

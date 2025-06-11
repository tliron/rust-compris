use super::super::context::*;

use bytestring::*;

//
// CommonResolveContext
//

/// Common resolve context.
#[derive(Clone, Debug, Default)]
pub struct CommonResolveContext {
    /// The optional source.
    pub source: Option<ByteString>,
}

impl CommonResolveContext {
    /// Constructor.
    pub fn new(source: Option<ByteString>) -> Self {
        Self { source }
    }
}

impl ResolveContext for CommonResolveContext {
    fn get_source(&self) -> Option<&ByteString> {
        self.source.as_ref()
    }
}

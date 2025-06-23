use super::super::{super::normal::*, *};

use std::io;

impl Parser {
    /// Parses XML into a [Value].
    pub fn read_xml<ReadT, AnnotationsT>(&self, _reader: &mut ReadT) -> Result<Value<AnnotationsT>, ParseError>
    where
        ReadT: io::Read,
    {
        todo!()
    }
}

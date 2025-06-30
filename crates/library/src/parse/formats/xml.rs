use super::super::{super::normal::*, *};

use std::io;

impl Parser {
    /// Parses XML into a [Value].
    pub fn read_xml<ReadT, AnnotatedT>(&self, _reader: &mut ReadT) -> Result<Value<AnnotatedT>, ParseError>
    where
        ReadT: io::Read,
    {
        todo!()
    }
}

use super::super::{super::normal::*, *};

use std::io;

impl Parser {
    /// Parses from XML into a normal value.
    pub fn read_xml<ReadT>(&self, _reader: &mut ReadT) -> Result<Value, ParseError>
    where
        ReadT: io::Read,
    {
        todo!()
    }
}

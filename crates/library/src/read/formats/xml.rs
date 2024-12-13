use super::super::{super::*, *};

use std::io::Read;

impl Reader {
    /// Reads from XML into a normal value.
    pub fn read_xml<R: Read>(&self, _reader: &mut R) -> Result<Value, ReadError> {
        todo!()
    }
}

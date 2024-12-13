use super::super::{super::normal::*, *};

use std::io::Read;

//
// Reader
//

impl Reader {
    /// Reads from XML into a normal value.
    pub fn read_xml<R: Read>(&self, _reader: &mut R) -> Result<Value, ReadError> {
        todo!()
    }
}

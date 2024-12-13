use super::super::{super::normal::*, *};

use std::io::Read;

//
// Reader
//

impl Reader {
    /// Reads from XML into a normal value.
    pub fn read_xml<ReadT>(&self, _reader: &mut ReadT) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
        todo!()
    }
}

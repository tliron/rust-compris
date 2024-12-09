use super::super::{super::*, *};

use std::io::Read;

impl<R: Read> Reader<R> {
    /// Reads from XML into a normal value.
    pub fn read_xml(&mut self) -> Result<Value, ReadError> {
        todo!()
    }
}

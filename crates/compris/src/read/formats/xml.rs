use super::super::{super::*, errors::*, reader::*};

use std::io::Read;

impl<R: Read> Reader<R> {
    pub fn read_xml(&mut self) -> Result<Value, ReadError> {
        todo!()
    }
}

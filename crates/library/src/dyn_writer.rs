use std::io::*;

//
// DynWriter
//

/// Provides a concrete [Write] implementation for dyn [Write].
pub struct DynWriter<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> DynWriter<'a> {
    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self { writer }
    }
}

impl<'a> Write for DynWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

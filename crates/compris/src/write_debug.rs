use super::styles::*;

use std::io;

//
// WriteDebug
//

/// Write a debug representation that can have multiple lines with nested indentation.
pub trait WriteDebug {
    /// Expected behavior:
    ///
    /// 1. Representations may include newlines, but note that they should not *end* in a newline.
    /// 2. If indentation is not zero, then all lines *after* the first (but *not* the first)
    ///    should start with that indentation.
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error>;

    fn write_debug(&self, writer: &mut dyn io::Write) -> Result<(), io::Error> {
        Self::write_debug_representation(&self, writer, 0, &Styles::new())?;
        writeln!(writer)
    }
}

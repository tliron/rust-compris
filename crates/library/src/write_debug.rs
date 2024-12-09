use super::styles::*;

use {kutil_std::*, std::io::*};

//
// WriteDebug
//

/// Write a debug representation that can have multiple lines with nested indentation
/// and colorization.
pub trait WriteDebug<W: Write> {
    /// Expected behavior by implementations:
    ///
    /// 1. Representations may include newlines, but note that they should not *end* in a newline.
    /// 2. If indentation is not zero, then all lines *after* the first (but *not* the first)
    ///    should start with that indentation.
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<()>;

    /// Write the debug representation with default styles.
    fn write_debug(&self, writer: &mut W) -> Result<()> {
        Self::write_debug_representation(&self, writer, 0, &Styles::default())?;
        writeln!(writer)
    }
}

//
// WriteDebugDyn
//

/// A version of [WriteDebug] for dyn [Write].
pub trait WriteDebugDyn<'a> {
    /// Write the debug representation with default styles.
    fn write_debug_dyn(&'a self, writer: &'a mut dyn Write) -> Result<()>;
}

impl<'a, T: WriteDebug<DynWriter<'a>>> WriteDebugDyn<'a> for T {
    fn write_debug_dyn(&'a self, writer: &'a mut dyn Write) -> Result<()> {
        let mut writer = DynWriter::new(writer);
        self.write_debug(&mut writer)
    }
}

use super::styles::*;

use {kutil_std::*, std::io::*};

//
// WriteDebug
//

/// Write a debug representation that can have multiple lines with nested indentation
/// and colorization.
pub trait WriteDebug<W: Write> {
    /// Expected behavior for implementations:
    ///
    /// 1. Representations may include newlines, but note that they should not *end* in a newline.
    /// 2. If indentation is not zero, then all lines *after* the first (but *not* the first)
    ///    should start with that indentation.
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<()>;

    /// Write the debug representation with default styles and a final newline.
    fn write_debug(&self, writer: &mut W) -> Result<()> {
        Self::write_debug_representation(&self, writer, 0, &Styles::default())?;
        writeln!(writer)
    }
}

//
// PrintDebug
//

/// [WriteDebug] to [stdout].
pub trait PrintDebug {
    /// Print the debug representation with default styles.
    fn print_debug(&self);
}

impl<T: WriteDebug<Stdout>> PrintDebug for T {
    fn print_debug(&self) {
        Self::write_debug(&self, &mut stdout()).unwrap();
    }
}

//
// PrintDebugAnstream
//

/// [WriteDebug] to [anstream::stdout].
pub trait PrintDebugAnstream {
    /// Print the debug representation with default styles.
    fn print_debug(&self);
}

impl<T: WriteDebug<anstream::Stdout>> PrintDebugAnstream for T {
    fn print_debug(&self) {
        let mut writer = anstream::stdout();
        Self::write_debug(&self, &mut writer).unwrap();
    }
}

//
// EprintDebugAnstream
//

/// [WriteDebug] to [anstream::stderr].
pub trait EprintDebugAnstream {
    /// Print the debug representation with default styles.
    fn eprint_debug(&self);
}

impl<T: WriteDebug<anstream::Stderr>> EprintDebugAnstream for T {
    fn eprint_debug(&self) {
        let mut writer = anstream::stderr();
        Self::write_debug(&self, &mut writer).unwrap();
    }
}

//
// ToDebugString
//

/// The [WriteDebug] representation as a string with no styles.
pub trait ToDebugString {
    /// Capture [WriteDebug::write_debug_representation] in a string.
    fn to_debug_string(&self) -> Result<String>;
}

impl<T: WriteDebug<BufWriter<Vec<u8>>>> ToDebugString for T {
    fn to_debug_string(&self) -> Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        self.write_debug_representation(&mut writer, 0, &Styles::none())?;
        match String::from_utf8(writer.buffer().into()) {
            Ok(string) => Ok(string),
            Err(err) => Err(Error::new(ErrorKind::Other, format!("{}", err))),
        }
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

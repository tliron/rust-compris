use super::super::normal::*;

use {kutil_cli::debug::*, owo_colors::*, std::io};

//
// Citation
//

/// How to find something.
#[derive(Debug, Clone, Default)]
pub struct Citation {
    /// Optional source.
    pub source: Option<String>,

    /// Optional coordinates.
    pub coordinates: Option<Coordinates>,

    /// Optional path.
    pub path: Option<String>,
}

impl Citation {
    /// Constructor.
    pub fn new(source: Option<String>, coordinates: Option<Coordinates>, path: Option<String>) -> Self {
        Self { source, coordinates, path }
    }

    /// True if [Debuggable::write_debug_representation] will write something.
    pub fn has_debug_representation(&self) -> bool {
        self.path.is_some() || self.coordinates.is_some()
    }
}

impl Debuggable for Citation {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        styles: &Styles,
    ) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let has_path = self.path.is_some();
        if has_path || self.coordinates.is_some() {
            if let Some(path) = &self.path {
                write!(writer, "{}", path.style(styles.meta))?;
            }

            if let Some(coordinates) = &self.coordinates {
                if has_path {
                    write!(writer, " ")?;
                }
                coordinates.write_debug_representation(writer, prefix, styles)?;
            }
        }

        Ok(())
    }
}

//
// Citable
//

/// Has a [Citation].
pub trait Citable: Sized {
    /// Gets the citation.
    fn get_citation(&self) -> &Citation;

    /// Sets the citation.
    fn with_citation(self, citation: Citation) -> Self;
}

//
// ToCited
//

/// Convert to a cited version.
pub trait ToCited<'a, CitedT>
where
    CitedT: 'a,
{
    /// Convert to cited version.
    fn to_cited(&'a self) -> CitedT;
}

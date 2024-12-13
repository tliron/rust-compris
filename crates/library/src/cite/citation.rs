use super::super::meta::*;

use {kutil_cli::debug::*, std::io};

//
// Citation
//

/// How to find something.
#[derive(Debug, Clone, Default)]
pub struct Citation {
    /// Optional name of source.
    pub source: Option<String>,

    /// Optional location in source.
    pub meta: Option<Meta>,

    /// Optional path in source.
    pub path: Option<String>,
}

impl Citation {
    /// Constructor.
    pub fn new(source: Option<String>, location: Option<Location>, path: Option<String>) -> Self {
        let meta = location.map(|l| Meta::new().with_location(Some(l)));
        Self { source, meta, path }
    }

    /// True if [Debuggable::write_debug_representation] will write something.
    pub fn has_debug_representation(&self) -> bool {
        self.path.is_some() || self.meta.is_some()
    }
}

impl Debuggable for Citation {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        let has_path = self.path.is_some();
        if has_path || self.meta.is_some() {
            if let Some(path) = &self.path {
                write!(writer, "{}", context.theme.meta.style(path))?;
            }

            if let Some(meta) = &self.meta {
                if let Some(location) = &meta.location {
                    if has_path {
                        write!(writer, " ")?;
                    }
                    location.write_debug_for(writer, context)?;
                }
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
    fn get_citation(&self) -> Option<&Citation>;

    /// Gets the citation.
    fn get_citation_mut(&mut self) -> Option<&mut Citation>;

    /// Sets the citation.
    fn with_citation(mut self, citation: Citation) -> Self {
        if let Some(our_citation) = self.get_citation_mut() {
            *our_citation = citation;
        }
        self
    }
}

//
// CitableFields
//

/// Has a [Citation] for its fields.
pub trait CitableFields {
    /// Get a field citation.
    fn get_field_citation(&self, name: &str) -> Option<&Citation>;
}

//
// ToCited
//

/// Convert to a version with a [Citation].
pub trait ToCited<'own, CitedT>
where
    CitedT: 'own,
{
    /// Convert to a version with a [Citation].
    fn to_cited(&'own self) -> CitedT;
}

use super::super::{meta::*, path::*};

use {kutil_cli::debug::*, std::io};

//
// Citation
//

/// How to find something.
#[derive(Clone, Debug, Default)]
pub struct Citation {
    /// Optional name of source.
    pub source: Option<String>,

    /// Optional location in source.
    pub meta: Option<Meta>,

    /// Optional path in source.
    pub path: Option<Path>,
}

impl Citation {
    /// Constructor.
    pub fn new(source: Option<String>, location: Option<Location>, path: Option<Path>) -> Self {
        let meta = location.map(|l| Meta::new().with_location(Some(l)));
        Self { source, meta, path }
    }

    /// True if [Debuggable::write_debug_for] will write something.
    pub fn has_debug(&self) -> bool {
        self.meta.is_some() || self.path.is_some()
    }

    /// Clone and add map key to path.
    pub fn with_map_key(&self, key: String) -> Citation {
        let mut path = match &self.path {
            Some(path) => path.clone(),
            None => Path::new(),
        };

        path.push_map_key(key);
        Citation { source: self.source.clone(), meta: self.meta.clone(), path: Some(path) }
    }

    /// Clone and add list index to path.
    pub fn with_list_index(&self, index: usize) -> Citation {
        let mut path = match &self.path {
            Some(path) => path.clone(),
            None => Path::new(),
        };

        path.push_list_index(index);
        Citation { source: self.source.clone(), meta: self.meta.clone(), path: Some(path) }
    }
}

impl Debuggable for Citation {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        if let Some(path) = &self.path {
            path.write_debug_for(writer, context)?;
        }

        if let Some(meta) = &self.meta {
            if let Some(location) = &meta.location {
                location.write_debug_for(writer, context)?;
            }
        }

        Ok(())
    }
}

//
// ToCited
//

/// Convert to a [Debuggable] with a [Citation].
pub trait ToCited<'own, CitedT>
where
    CitedT: Debuggable + 'own,
{
    /// Convert to a version with a [Citation].
    fn to_cited(&'own self) -> CitedT;
}

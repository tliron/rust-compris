use super::super::normal::*;

use std::fmt;

//
// PathElementKind
//

/// Path element kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathElementKind<'own> {
    /// List index.
    ListIndex(usize),

    /// Map key.
    MapKey(&'own Value),
}

//
// PathElement
//

/// Path element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathElement<'own> {
    /// Value.
    pub value: &'own Value,

    /// Kind.
    pub kind: Option<PathElementKind<'own>>,
}

impl<'own> PathElement<'own> {
    /// Constructor.
    pub fn new(value: &'own Value, kind: Option<PathElementKind<'own>>) -> Self {
        Self { value, kind }
    }
}

impl<'own> fmt::Display for PathElement<'own> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(kind) = &self.kind {
            match kind {
                PathElementKind::ListIndex(index) => write!(formatter, "[{}]", index)?,

                PathElementKind::MapKey(key) => {
                    let key = key.to_string();
                    if key.contains("\"") || key.contains(char::is_whitespace) {
                        // Debug will quote the string
                        fmt::Debug::fmt(&key, formatter)?
                    } else {
                        fmt::Display::fmt(&key, formatter)?
                    }
                }
            }
        }
        Ok(())
    }
}

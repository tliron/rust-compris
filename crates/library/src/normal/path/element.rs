use super::super::value::*;

use std::fmt;

//
// PathElement
//

/// Path element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathElement<'a> {
    /// Value.
    pub value: &'a Value,

    /// Kind.
    pub kind: Option<PathElementKind<'a>>,
}

impl<'a> PathElement<'a> {
    /// Constructor.
    pub fn new(value: &'a Value, kind: Option<PathElementKind<'a>>) -> Self {
        Self { value, kind }
    }
}

impl<'a> fmt::Display for PathElement<'a> {
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

//
// PathElementKind
//

/// Path element kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathElementKind<'a> {
    /// List index.
    ListIndex(usize),

    /// Map key.
    MapKey(&'a Value),
}

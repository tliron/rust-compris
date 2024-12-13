use super::{super::normal::*, element::*};

use {
    kutil_std::iter::*,
    std::{
        fmt::{self, Write},
        ptr,
    },
};

//
// Path
//

/// Path.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path<'own> {
    /// Path elements.
    pub elements: Vec<PathElement<'own>>,
}

impl<'own> Path<'own> {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// True if the path is a line (length >= 2).
    pub fn is_linear(&self) -> bool {
        self.elements.len() >= 2
    }

    /// Find the path from an ancestor to a descendent, if it exists.
    ///
    /// Paths will include the endpoints. In the case of the path from oneself to
    /// oneself, it will contains just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided values are
    /// the *pointers* represented by the references. Thus a clone of a value or an
    /// otherwise equal value will *not* be considered identical.
    pub fn find(ancestor: &'own Value, descendent: &'own Value) -> Option<Self> {
        if ptr::eq(descendent, ancestor) {
            let mut path = Path::new();
            path.push(ancestor);
            return Some(path);
        }

        match ancestor {
            Value::List(list) => {
                for (index, child) in list.value.iter().enumerate() {
                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut path = Path::new();
                        path.push_list_index(ancestor, index);
                        path.extend(child_path);
                        return Some(path);
                    }
                }
            }

            Value::Map(map) => {
                for (key, child) in &map.value {
                    // The descendent we are looking for might be this key
                    if ptr::eq(descendent, key) {
                        let mut path = Path::new();
                        path.push_map_key(ancestor, key);
                        return Some(path);
                    }

                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut path = Path::new();
                        path.push_map_key(ancestor, key);
                        path.extend(child_path);
                        return Some(path);
                    }
                }
            }

            _ => {}
        }

        None
    }

    /// Push a new path element.
    pub fn push(&mut self, value: &'own Value) {
        self.elements.push(PathElement::new(value, None))
    }

    /// Push a new list index path element.
    pub fn push_list_index(&mut self, value: &'own Value, index: usize) {
        self.elements.push(PathElement::new(value, Some(PathElementKind::ListIndex(index))))
    }

    /// Push a new map key path element.
    pub fn push_map_key(&mut self, value: &'own Value, key: &'own Value) {
        self.elements.push(PathElement::new(value, Some(PathElementKind::MapKey(key))))
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: Path<'own>) {
        self.elements.extend(other.elements);
    }
}

impl<'own> fmt::Display for Path<'own> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (element, first) in IterateWithFirst::new(&self.elements) {
            if !first {
                if let Some(kind) = &element.kind {
                    if let PathElementKind::MapKey(_) = kind {
                        formatter.write_char('.')?;
                    }
                }
            }

            fmt::Display::fmt(element, formatter)?;
        }
        Ok(())
    }
}

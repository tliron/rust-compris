use super::{super::value::*, element::*};

use std::{
    fmt::{self, Write},
    ptr,
};

//
// Path
//

/// Path.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path<'a> {
    /// Path elements.
    pub elements: Vec<PathElement<'a>>,
}

impl<'a> Path<'a> {
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
    pub fn find(ancestor: &'a Value, descendent: &'a Value) -> Option<Self> {
        if ptr::eq(descendent, ancestor) {
            let mut path = Path::new();
            path.push(ancestor);
            return Some(path);
        }

        match ancestor {
            Value::List(list) => {
                for (index, element) in list.value.iter().enumerate() {
                    if let Some(child_path) = Self::find(element, descendent) {
                        let mut path = Path::new();
                        path.push_list_index(ancestor, index);
                        path.extend(child_path);
                        return Some(path);
                    }
                }
            }

            Value::Map(map) => {
                for (key, value) in &map.value {
                    // The descendent we are looking for might be this key
                    if ptr::eq(descendent, key) {
                        let mut path = Path::new();
                        path.push_map_key(ancestor, key);
                        return Some(path);
                    }

                    if let Some(child_path) = Self::find(value, descendent) {
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
    pub fn push(&mut self, value: &'a Value) {
        self.elements.push(PathElement::new(value, None))
    }

    /// Push a new list index path element.
    pub fn push_list_index(&mut self, value: &'a Value, index: usize) {
        self.elements.push(PathElement::new(value, Some(PathElementKind::ListIndex(index))))
    }

    /// Push a new map key path element.
    pub fn push_map_key(&mut self, value: &'a Value, key: &'a Value) {
        self.elements.push(PathElement::new(value, Some(PathElementKind::MapKey(key))))
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: Path<'a>) {
        self.elements.extend(other.elements);
    }
}

impl<'a> fmt::Display for Path<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for element in &self.elements {
            if first {
                first = false;
            } else {
                if let Some(kind) = &element.kind {
                    if let PathElementKind::MapKey(_) = kind {
                        formatter.write_char('.')?;
                    }
                }
            }

            element.fmt(formatter)?;
        }
        Ok(())
    }
}

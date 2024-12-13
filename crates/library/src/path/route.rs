use super::{super::normal::*, node::*, path::*, segment::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        fmt::{self, Write},
        io, ptr,
    },
};

//
// Route
//

/// Route between two [Value] nodes.
///
/// Because this type contains references to the values, it shares their lifetime.
/// For a version of [Route] that does not keep the references, see [Path].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Route<'own> {
    /// Route nodes.
    pub nodes: Vec<PathNode<'own>>,
}

impl<'own> Route<'own> {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the route from an ancestor to a descendent, if it exists.
    ///
    /// Routes will include the endpoints. In the case of the route from oneself to
    /// oneself, it will contains just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided values are
    /// the *pointers* represented by the references. Thus a clone of a value or an
    /// otherwise equal value will *not* be considered identical.
    pub fn find(ancestor: &'own Value, descendent: &'own Value) -> Option<Self> {
        if ptr::eq(descendent, ancestor) {
            let mut path = Route::new();
            path.push(ancestor);
            return Some(path);
        }

        match ancestor {
            Value::List(list) => {
                for (index, child) in list.value.iter().enumerate() {
                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut path = Route::new();
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
                        let mut path = Route::new();
                        path.push_map_key(ancestor, key);
                        return Some(path);
                    }

                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut path = Route::new();
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

    /// Push a new route node.
    pub fn push(&mut self, value: &'own Value) {
        self.nodes.push(PathNode::new(value, None))
    }

    /// Push a new list index route node.
    pub fn push_list_index(&mut self, value: &'own Value, index: usize) {
        self.nodes.push(PathNode::new(value, Some(PathSegment::ListIndex(index))))
    }

    /// Push a new map key route node.
    pub fn push_map_key(&mut self, value: &'own Value, key: &'own Value) {
        self.nodes.push(PathNode::new(value, Some(PathSegment::MapKey(key))))
    }

    /// Extend this route with another route.
    pub fn extend(&mut self, other: Route<'own>) {
        self.nodes.extend(other.nodes);
    }

    /// To [Path].
    pub fn to_path(&self) -> Path {
        Path {
            segments: self
                .nodes
                .iter()
                .filter_map(|n| match &n.segment {
                    Some(segment) => Some(segment.to_string_keys()),
                    None => None,
                })
                .collect(),
        }
    }
}

impl<'own> Debuggable for Route<'own> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for (node, first) in IterateWithFirst::new(&self.nodes) {
            if let Some(segment) = &node.segment {
                if !first && matches!(segment, PathSegment::MapKey(_)) {
                    context.theme.write_delimiter(writer, ".")?;
                }

                segment.write_debug_for(writer, context)?;
            }
        }

        Ok(())
    }
}

impl<'own> fmt::Display for Route<'own> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (node, first) in IterateWithFirst::new(&self.nodes) {
            if let Some(segment) = &node.segment {
                if !first && matches!(segment, PathSegment::MapKey(_)) {
                    formatter.write_char('.')?;
                }

                fmt::Display::fmt(segment, formatter)?;
            }
        }

        Ok(())
    }
}

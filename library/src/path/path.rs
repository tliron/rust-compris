use super::{super::normal::*, node::*, representation::*, segment::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        fmt::{self, Write},
        io, ptr,
    },
};

//
// Path
//

/// Path between two [Variant] nodes.
///
/// Because this type contains references to the variants, it shares their lifetime. For a version of
/// [Path] that does not keep the references see [PathRepresentation].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path<'own, AnnotatedT> {
    /// Path nodes.
    pub nodes: Vec<PathNode<'own, AnnotatedT>>,
}

impl<'own, AnnotatedT> Path<'own, AnnotatedT> {
    /// Constructor.
    pub fn new() -> Self
    where
        AnnotatedT: Default,
    {
        Self::default()
    }

    /// Find the path from an ancestor to a descendent, if it exists.
    ///
    /// Paths will include the endpoints. In the case of the route from oneself to oneself, it will
    /// contain just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided variants are the
    /// *pointers* represented by the references. Thus a clone of a variant or an otherwise equal
    /// variant will *not* be considered identical.
    pub fn find(ancestor: &'own Variant<AnnotatedT>, descendent: &'own Variant<AnnotatedT>) -> Option<Self>
    where
        AnnotatedT: Default,
    {
        if ptr::eq(descendent, ancestor) {
            let mut route = Path::new();
            route.push(ancestor);
            return Some(route);
        }

        match ancestor {
            Variant::List(list) => {
                for (index, child) in list.inner.iter().enumerate() {
                    if let Some(child_route) = Self::find(child, descendent) {
                        let mut route = Path::new();
                        route.push_list_index(ancestor, index);
                        route.extend(child_route);
                        return Some(route);
                    }
                }
            }

            Variant::Map(map) => {
                for (key, child) in &map.inner {
                    // The descendent we are looking for might be this key
                    if ptr::eq(descendent, key) {
                        let mut route = Path::new();
                        route.push_map_key(ancestor, key);
                        return Some(route);
                    }

                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut route = Path::new();
                        route.push_map_key(ancestor, key);
                        route.extend(child_path);
                        return Some(route);
                    }
                }
            }

            _ => {}
        }

        None
    }

    /// Push a new path node.
    pub fn push(&mut self, variant: &'own Variant<AnnotatedT>) {
        self.nodes.push(PathNode::new(variant, None))
    }

    /// Push a new list index path node.
    pub fn push_list_index(&mut self, variant: &'own Variant<AnnotatedT>, index: usize) {
        self.nodes.push(PathNode::new(variant, Some(PathSegment::ListIndex(index))))
    }

    /// Push a new map key path node.
    pub fn push_map_key(&mut self, variant: &'own Variant<AnnotatedT>, key: &'own Variant<AnnotatedT>) {
        self.nodes.push(PathNode::new(variant, Some(PathSegment::MapKey(key))))
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: Path<'own, AnnotatedT>) {
        self.nodes.extend(other.nodes);
    }

    /// Into [PathRepresentation].
    pub fn into_representation(self) -> PathRepresentation {
        PathRepresentation {
            segments: self
                .nodes
                .into_iter()
                .filter_map(|node| node.segment.map(|segment| segment.to_string_keys()))
                .collect(),
        }
    }
}

impl<'own, AnnotatedT> Debuggable for Path<'own, AnnotatedT> {
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

impl<'own, AnnotatedT> fmt::Display for Path<'own, AnnotatedT> {
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

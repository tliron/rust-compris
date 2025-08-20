use super::{super::normal::*, path::*, segment::*};

use {
    kutil::{
        cli::depict::*,
        std::{iter::*, immutable::*},
    },
    std::{
        fmt::{self, Write},
        io,
    },
};

//
// PathRepresentation
//

/// Path between two [Variant] nodes.
///
/// This type does not keep the references to the variants. For a version that does, see [Path].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathRepresentation {
    /// Path segments.
    pub segments: Vec<PathSegment<ByteString>>,
}

impl PathRepresentation {
    /// Find the path from an ancestor to a descendent, if it exists.
    ///
    /// Paths will include the endpoints. In the case of the route from oneself to oneself, it will
    /// contain just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided variants are the
    /// *pointers* represented by the references. Thus a clone of a variant or an otherwise equal
    /// variant will *not* be considered identical.
    pub fn find<'own, AnnotatedT>(
        ancestor: &'own Variant<AnnotatedT>,
        descendent: &'own Variant<AnnotatedT>,
    ) -> Option<Self>
    where
        AnnotatedT: Default,
    {
        let route = Path::find(ancestor, descendent)?;
        let path = route.into_representation();
        if !path.segments.is_empty() { Some(path) } else { None }
    }

    /// Push a new list index path segment.
    pub fn push_list_index(&mut self, index: usize) {
        self.segments.push(PathSegment::ListIndex(index));
    }

    /// Push a new map key path segment.
    pub fn push_map_key(&mut self, key: ByteString) {
        self.segments.push(PathSegment::MapKey(key));
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: PathRepresentation) {
        self.segments.extend(other.segments);
    }
}

impl Depict for PathRepresentation {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for (segment, first) in IterateWithFirst::new(&self.segments) {
            if !first && matches!(segment, PathSegment::MapKey(_)) {
                context.theme.write_delimiter(writer, '.')?;
            }

            segment.depict(writer, context)?;
        }

        Ok(())
    }
}

impl fmt::Display for PathRepresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (segment, first) in IterateWithFirst::new(&self.segments) {
            if !first && matches!(segment, PathSegment::MapKey(_)) {
                formatter.write_char('.')?;
            }

            fmt::Display::fmt(segment, formatter)?;
        }

        Ok(())
    }
}

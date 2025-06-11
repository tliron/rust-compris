use super::{super::normal::*, route::*, segment::*};

use {
    bytestring::*,
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{
        fmt::{self, Write},
        io,
    },
};

//
// Path
//

/// Path between two [Value] nodes.
///
/// This type does not keep the references to the values.
/// For a version of that does, see [Route].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path {
    /// Path segments.
    pub segments: Vec<PathSegment<ByteString>>,
}

impl Path {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the path from an ancestor to a descendent, if it exists.
    ///
    /// Paths will include the endpoints. In the case of the path from oneself to
    /// oneself, it will contains just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided values are
    /// the *pointers* represented by the references. Thus a clone of a value or an
    /// otherwise equal value will *not* be considered identical.
    pub fn find<'own>(ancestor: &'own Value, descendent: &'own Value) -> Option<Self> {
        let route = Route::find(ancestor, descendent)?;
        let path = route.to_path();
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
    pub fn extend(&mut self, other: Path) {
        self.segments.extend(other.segments);
    }
}

impl Debuggable for Path {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for (segment, first) in IterateWithFirst::new(&self.segments) {
            if !first && matches!(segment, PathSegment::MapKey(_)) {
                context.theme.write_delimiter(writer, ".")?;
            }

            segment.write_debug_for(writer, context)?;
        }

        Ok(())
    }
}

impl fmt::Display for Path {
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

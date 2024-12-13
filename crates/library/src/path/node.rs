use super::{super::normal::*, segment::*};

//
// PathNode
//

/// Path node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathNode<'own> {
    /// Value.
    pub value: &'own Value,

    /// Segment.
    pub segment: Option<PathSegment<&'own Value>>,
}

impl<'own> PathNode<'own> {
    /// Constructor.
    pub fn new(value: &'own Value, segment: Option<PathSegment<&'own Value>>) -> Self {
        Self { value, segment }
    }
}

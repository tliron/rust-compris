use super::{super::normal::*, segment::*};

//
// PathNode
//

/// [Path] node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathNode<'own, AnnotationsT> {
    /// Value.
    pub value: &'own Value<AnnotationsT>,

    /// Segment.
    pub segment: Option<PathSegment<&'own Value<AnnotationsT>>>,
}

impl<'own, AnnotationsT> PathNode<'own, AnnotationsT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotationsT>, segment: Option<PathSegment<&'own Value<AnnotationsT>>>) -> Self {
        Self { value, segment }
    }
}

use super::{super::normal::*, segment::*};

//
// PathNode
//

/// [Path] node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathNode<'own, AnnotatedT> {
    /// Value.
    pub value: &'own Value<AnnotatedT>,

    /// Segment.
    pub segment: Option<PathSegment<&'own Value<AnnotatedT>>>,
}

impl<'own, AnnotatedT> PathNode<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(value: &'own Value<AnnotatedT>, segment: Option<PathSegment<&'own Value<AnnotatedT>>>) -> Self {
        Self { value, segment }
    }
}

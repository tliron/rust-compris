use super::{super::normal::*, segment::*};

//
// PathNode
//

/// [Path] node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathNode<'own, AnnotatedT> {
    /// Variant.
    pub variant: &'own Variant<AnnotatedT>,

    /// Segment.
    pub segment: Option<PathSegment<&'own Variant<AnnotatedT>>>,
}

impl<'own, AnnotatedT> PathNode<'own, AnnotatedT> {
    /// Constructor.
    pub fn new(variant: &'own Variant<AnnotatedT>, segment: Option<PathSegment<&'own Variant<AnnotatedT>>>) -> Self {
        Self { variant, segment }
    }
}

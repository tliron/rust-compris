use super::{annotated::*, annotations::*};

use {
    kutil::cli::depict::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Annotate
//

/// Wrapper that adds an [Annotated] implementation.
///
/// Useful for wrapping fields of structs when using [Resolve](super::super::resolve::Resolve),
/// as it will retain the [Annotations] of the resolved [Variant](super::super::normal::Variant).
#[derive(Clone, Debug, Default)]
pub struct Annotate<InnerT, AnnotatedT> {
    /// Inner.
    pub inner: InnerT,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl<InnerT, AnnotatedT> Annotate<InnerT, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self
    where
        AnnotatedT: Default,
    {
        Self { inner, annotated: Default::default() }
    }
}

impl<InnerT, AnnotatedT> Annotated for Annotate<InnerT, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.annotated.annotations()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotated.annotations_mut()
    }
}

// Delegated

impl<InnerT, AnnotatedT> Depict for Annotate<InnerT, AnnotatedT>
where
    InnerT: Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        self.inner.depict(writer, context)
    }
}

impl<InnerT, AnnotatedT> PartialEq for Annotate<InnerT, AnnotatedT>
where
    InnerT: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<InnerT, AnnotatedT> Eq for Annotate<InnerT, AnnotatedT> where InnerT: Eq {}

impl<InnerT, AnnotatedT> PartialOrd for Annotate<InnerT, AnnotatedT>
where
    InnerT: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<InnerT, AnnotatedT> Ord for Annotate<InnerT, AnnotatedT>
where
    InnerT: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<InnerT, AnnotatedT> Hash for Annotate<InnerT, AnnotatedT>
where
    InnerT: Hash,
{
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.inner.hash(state);
    }
}

impl<InnerT, AnnotatedT> fmt::Display for Annotate<InnerT, AnnotatedT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

// Conversions

impl<InnerT, AnnotatedT> From<InnerT> for Annotate<InnerT, AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(inner: InnerT) -> Self {
        Self::new(inner)
    }
}

use super::super::annotation::*;

use {
    kutil_cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Null
//

/// Normal null value.
///
/// Annotations, if present, are *ignored* for the purposes of comparison and hashing.
#[derive(Clone, Debug)]
pub struct Null<AnnotationsT> {
    /// Annotations.
    pub annotations: AnnotationsT,
}

impl<AnnotationsT> Null<AnnotationsT> {
    /// Removes all [Annotations].
    pub fn without_annotations(self) -> Null<WithoutAnnotations> {
        Null::default()
    }

    /// Into different annotations.
    pub fn into_annotated<NewAnnotationsT>(self) -> Null<NewAnnotationsT>
    where
        AnnotationsT: Annotated,
        NewAnnotationsT: Annotated + Default,
    {
        if AnnotationsT::is_annotated()
            && NewAnnotationsT::is_annotated()
            && let Some(annotations) = self.annotations.get_annotations()
        {
            Null::default().with_annotations(annotations.clone())
        } else {
            Null::default()
        }
    }
}

impl<AnnotationsT> Annotated for Null<AnnotationsT>
where
    AnnotationsT: Annotated,
{
    fn is_annotated() -> bool {
        AnnotationsT::is_annotated()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        self.annotations.get_annotations()
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.annotations.get_annotations_mut()
    }

    fn set_annotations(&mut self, annotations: Annotations) {
        self.annotations.set_annotations(annotations);
    }
}

impl<AnnotationsT> Debuggable for Null<AnnotationsT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, "Null")
    }
}

impl<AnnotationsT> Default for Null<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn default() -> Self {
        Self { annotations: AnnotationsT::default() }
    }
}

impl<AnnotationsT> fmt::Display for Null<AnnotationsT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("Null", formatter)
    }
}

// Basics

impl<AnnotationsT> PartialEq for Null<AnnotationsT> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<AnnotationsT> Eq for Null<AnnotationsT> {}

impl<AnnotationsT> PartialOrd for Null<AnnotationsT> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<AnnotationsT> Ord for Null<AnnotationsT> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<AnnotationsT> Hash for Null<AnnotationsT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        ().hash(state)
    }
}

// Conversions

impl<AnnotationsT> From<()> for Null<AnnotationsT>
where
    AnnotationsT: Default,
{
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<AnnotationsT> From<Null<AnnotationsT>> for () {
    fn from(_: Null<AnnotationsT>) -> Self {
        ()
    }
}

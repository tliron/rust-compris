use super::{super::annotated::*, mode::*};

use {
    kutil::cli::depict::*,
    std::{error::*, io},
};

//
// AnnotatedDepiction
//

/// A [Depict] wrapper for an [Annotated] [Depict].
///
/// The inner [Depict] is called first and the
/// [Annotations](super::super::annotations::Annotations) next.
pub struct AnnotatedDepiction<'own, InnerT> {
    /// Inner.
    pub inner: &'own InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'own, InnerT> AnnotatedDepiction<'own, InnerT> {
    /// Constructor.
    pub fn new(inner: &'own InnerT, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, InnerT> Depict for AnnotatedDepiction<'own, InnerT>
where
    InnerT: Annotated + Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.inner.annotations() {
            match self.mode {
                AnnotatedDepictionMode::Inline => {
                    self.inner.depict(writer, context)?;
                    if annotations.has_depiction(DepictionFormat::Compact) {
                        annotations.depict(writer, &context.clone().with_format(DepictionFormat::Compact))?;
                    }
                }

                AnnotatedDepictionMode::Multiline => {
                    if annotations.has_depiction(DepictionFormat::Optimized) {
                        annotations.depict(writer, &context.clone().with_format(DepictionFormat::Optimized))?;
                        context.indent(writer)?;
                    } else {
                        context.separate(writer)?;
                    }
                    self.inner.depict(writer, context)?;
                }
            }
        } else {
            self.inner.depict(writer, context)?;
        }

        Ok(())
    }
}

//
// ToAnnotatedDepiction
//

/// To [AnnotatedDepiction].
pub trait ToAnnotatedDepiction<'own>
where
    Self: Sized,
{
    /// To [AnnotatedDepiction].
    fn annotated_depiction(&'own self) -> AnnotatedDepiction<'own, Self>;
}

impl<'own, ErrorT> ToAnnotatedDepiction<'own> for ErrorT
where
    ErrorT: Error,
{
    fn annotated_depiction(&'own self) -> AnnotatedDepiction<'own, Self> {
        AnnotatedDepiction::new(self, AnnotatedDepictionMode::Multiline)
    }
}

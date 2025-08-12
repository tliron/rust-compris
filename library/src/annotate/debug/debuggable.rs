use super::{super::annotated::*, mode::*};

use {
    kutil::cli::debug::*,
    std::{error::*, io},
};

//
// AnnotatedDebuggable
//

/// A [Debuggable] implementation for an [Annotated] [Debuggable].
///
/// The [Debuggable] is written first and the [Annotations](super::super::annotations::Annotations)
/// next.
pub struct AnnotatedDebuggable<'own, InnerT> {
    /// Inner.
    pub inner: &'own InnerT,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,
}

impl<'own, InnerT> AnnotatedDebuggable<'own, InnerT> {
    /// Constructor.
    pub fn new(inner: &'own InnerT, mode: AnnotatedDebuggableMode) -> Self {
        Self { inner, mode }
    }
}

impl<'own, InnerT> Debuggable for AnnotatedDebuggable<'own, InnerT>
where
    InnerT: Annotated + Debuggable,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.inner.get_annotations() {
            match self.mode {
                AnnotatedDebuggableMode::Inline => {
                    self.inner.write_debug_for(writer, context)?;
                    if annotations.has_debug(DebugFormat::Compact) {
                        annotations.write_debug_for(writer, &context.clone().with_format(DebugFormat::Compact))?;
                    }
                }

                AnnotatedDebuggableMode::Multiline => {
                    if annotations.has_debug(DebugFormat::Reduced) {
                        annotations.write_debug_for(writer, &context.clone().with_format(DebugFormat::Reduced))?;
                        context.indent(writer)?;
                    } else {
                        context.separate(writer)?;
                    }
                    self.inner.write_debug_for(writer, context)?;
                }
            }
        } else {
            self.inner.write_debug_for(writer, context)?;
        }

        Ok(())
    }
}

//
// ToAnnotatedDebuggable
//

/// To [AnnotatedDebuggable].
pub trait ToAnnotatedDebuggable<'own>
where
    Self: Sized,
{
    /// To [AnnotatedDebuggable].
    fn annotated_debuggable(&'own self) -> AnnotatedDebuggable<'own, Self>;
}

impl<'own, ErrorT> ToAnnotatedDebuggable<'own> for ErrorT
where
    ErrorT: Error,
{
    fn annotated_debuggable(&'own self) -> AnnotatedDebuggable<'own, Self> {
        AnnotatedDebuggable::new(self, AnnotatedDebuggableMode::Multiline)
    }
}

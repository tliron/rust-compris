use super::{super::annotated::*, debuggable::*, mode::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{cmp::*, collections::*, error::*, io},
};

//
// AnnotatedDebuggables
//

/// A [Debuggable] implementation for an [Iterator] of [Annotated] [Debuggable].
pub struct AnnotatedDebuggables<'own, InnerT, ItemT>
where
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own,
{
    /// Inner.
    pub inner: &'own InnerT,

    /// Mode.
    pub mode: AnnotatedDebuggableMode,

    /// Optional heading.
    pub heading: Option<String>,
}

impl<'own, InnerT, ItemT> AnnotatedDebuggables<'own, InnerT, ItemT>
where
    ItemT: 'own,
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
{
    /// Constructor.
    pub fn new(inner: &'own InnerT, mode: AnnotatedDebuggableMode, heading: Option<String>) -> Self {
        Self { inner, mode, heading }
    }
}

impl<'own, InnerT, ItemT> Debuggable for AnnotatedDebuggables<'own, InnerT, ItemT>
where
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own + Annotated + Debuggable,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(heading) = &self.heading {
            context.theme.write_heading(writer, heading)?;
        }

        let mut table = BTreeMap::<_, Vec<_>>::new();
        for item in self.inner {
            let source = item.get_annotations().and_then(|annotations| annotations.source.clone());
            match table.get_mut(&source) {
                Some(list) => list.push(item),
                None => {
                    let mut list = Vec::new();
                    list.push(item);
                    table.insert(source, list);
                }
            }
        }

        table.values_mut().for_each(|list| {
            list.sort_by(|a, b| {
                if let Some(a_annotations) = a.get_annotations()
                    && let Some(a_span) = &a_annotations.span
                    && let Some(b_annotations) = b.get_annotations()
                    && let Some(b_span) = &b_annotations.span
                {
                    a_span.start.cmp(&b_span.start)
                } else {
                    Ordering::Equal
                }
            })
        });

        for ((source, list), first) in IterateWithFirst::new(table) {
            context.separate_or_indent(writer, first && self.heading.is_none())?;

            match source {
                Some(source) => context.theme.write_meta(writer, source)?,
                None => context.theme.write_meta(writer, "general")?,
            }

            for item in list {
                context.indent_into(writer, utils::DEBUG_INTO_LIST_ITEM)?;
                let child_context = context.clone().with_separator(true).increase_indentation();
                AnnotatedDebuggable::new(item, self.mode).write_debug_for(writer, &child_context)?;
            }
        }

        Ok(())
    }
}

//
// ToAnnotatedDebuggables
//

///
pub trait ToAnnotatedDebuggables<'own, ItemT>
where
    Self: 'own + Sized,
    ItemT: 'own,
    &'own Self: IntoIterator<Item = &'own ItemT>,
{
    /// [Debuggable](Debuggable) with [Annotations](super::super::annotations::Annotations).
    fn annotated_debuggables(&'own self, heading: Option<String>) -> AnnotatedDebuggables<'own, Self, ItemT>;
}

impl<'own, ErrorIterableT, ErrorT> ToAnnotatedDebuggables<'own, ErrorT> for ErrorIterableT
where
    ErrorIterableT: 'own,
    &'own ErrorIterableT: IntoIterator<Item = &'own ErrorT>,
    ErrorT: 'own + Error,
{
    fn annotated_debuggables(&'own self, heading: Option<String>) -> AnnotatedDebuggables<'own, Self, ErrorT> {
        AnnotatedDebuggables::new(self, AnnotatedDebuggableMode::Full, heading)
    }
}

use super::{super::annotated::*, depiction::*, mode::*};

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{cmp::*, collections::*, error::*, io},
};

//
// AnnotatedDepictions
//

/// A [Depict] wrapper for an [Iterator] of [Annotated] [Depict].
pub struct AnnotatedDepictions<'own, InnerT, ItemT>
where
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own,
{
    /// Inner.
    pub inner: &'own InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,

    /// Optional heading.
    pub heading: Option<String>,
}

impl<'own, InnerT, ItemT> AnnotatedDepictions<'own, InnerT, ItemT>
where
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own,
{
    /// Constructor.
    pub fn new(inner: &'own InnerT, mode: AnnotatedDepictionMode, heading: Option<String>) -> Self {
        Self { inner, mode, heading }
    }
}

impl<'own, InnerT, ItemT> Depict for AnnotatedDepictions<'own, InnerT, ItemT>
where
    &'own InnerT: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own + Annotated + Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(heading) = &self.heading {
            context.theme.write_heading(writer, heading)?;
        }

        let mut table = BTreeMap::<_, Vec<_>>::default();
        for item in self.inner {
            let source = item.annotations().and_then(|annotations| annotations.source.clone());
            match table.get_mut(&source) {
                Some(list) => list.push(item),
                None => {
                    let mut list = Vec::default();
                    list.push(item);
                    table.insert(source, list);
                }
            }
        }

        table.values_mut().for_each(|list| {
            list.sort_by(|a, b| {
                if let Some(a_annotations) = a.annotations()
                    && let Some(a_span) = &a_annotations.span
                    && let Some(b_annotations) = b.annotations()
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
                context.indent_into(writer, utils::DEPICT_INTO_LIST_ITEM)?;
                let child_context = context.clone().with_separator(true).increase_indentation();
                AnnotatedDepiction::new(item, self.mode).depict(writer, &child_context)?;
            }
        }

        Ok(())
    }
}

//
// ToAnnotatedDepictions
//

/// To [AnnotatedDepictions].
pub trait ToAnnotatedDepictions<'own, ItemT>
where
    Self: 'own + Sized,
    &'own Self: IntoIterator<Item = &'own ItemT>,
    ItemT: 'own,
{
    /// To [AnnotatedDepictions].
    fn annotated_depictions(&'own self, heading: Option<String>) -> AnnotatedDepictions<'own, Self, ItemT>;
}

impl<'own, ErrorIterableT, ErrorT> ToAnnotatedDepictions<'own, ErrorT> for ErrorIterableT
where
    ErrorIterableT: 'own,
    &'own ErrorIterableT: IntoIterator<Item = &'own ErrorT>,
    ErrorT: 'own + Error,
{
    fn annotated_depictions(&'own self, heading: Option<String>) -> AnnotatedDepictions<'own, Self, ErrorT> {
        AnnotatedDepictions::new(self, AnnotatedDepictionMode::Multiline, heading)
    }
}

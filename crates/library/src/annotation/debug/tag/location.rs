use super::super::super::fields::*;

use {kutil_cli::debug::*, std::io};

/// Location tag for [Debuggable](kutil_cli::debug::Debuggable).
pub fn location<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedFields,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name)
        && let Some(span) = &annotations.span
    {
        write!(writer, " {} {}", context.theme.delimiter("@"), context.theme.meta(&span.start))?;
    }

    Ok(())
}

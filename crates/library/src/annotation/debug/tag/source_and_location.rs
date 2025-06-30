use super::super::super::fields::*;

use {kutil_cli::debug::*, std::io};

/// Source and location tag for a [Debuggable](kutil_cli::debug::Debuggable).
pub fn source_and_location<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedFields,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name) {
        if annotations.source.is_some() || annotations.span.is_some() {
            write!(writer, " {} ", context.theme.delimiter("@"))?;
        }

        if let Some(source) = &annotations.source {
            context.theme.write_meta(writer, source)?;
        }

        if let Some(span) = &annotations.span {
            if annotations.source.is_some() {
                write!(writer, " ")?;
            }
            context.theme.write_meta(writer, &span.start)?;
        }
    }

    Ok(())
}

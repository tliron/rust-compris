use super::super::super::r#struct::*;

use {kutil_cli::debug::*, std::io};

/// Source tag for a [Debuggable](kutil_cli::debug::Debuggable).
pub fn source<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedStruct,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name)
        && let Some(source) = &annotations.source
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, "@")?;
        context.theme.write_meta(writer, source)?;
    }

    Ok(())
}

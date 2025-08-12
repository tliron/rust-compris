use super::super::super::r#struct::*;

use {kutil::cli::debug::*, std::io};

/// Source and span tag for a [Debuggable](kutil::cli::debug::Debuggable).
pub fn source_and_span<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedStruct,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name) {
        annotations.write_debug_for(writer, &context.child().with_format(DebugFormat::Compact))?;
    }

    Ok(())
}

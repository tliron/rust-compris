use super::super::super::r#struct::*;

use {kutil::cli::depict::*, std::io};

/// Source and span tag for a [Depict](kutil::cli::depict::Depict).
pub fn source_and_span<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedStruct,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name) {
        annotations.depict(writer, &context.child().with_format(DepictionFormat::Compact))?;
    }

    Ok(())
}

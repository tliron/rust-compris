use super::super::super::{DEPICT_ANNOTATIONS_PREFIX, r#struct::*};

use {kutil::cli::depict::*, std::io};

/// Source tag for a [Depict](kutil::cli::depict::Depict).
pub fn source<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedStruct,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.get_field_annotations(field_name)
        && let Some(source) = &annotations.source
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_PREFIX)?;
        context.theme.write_meta(writer, source)?;
    }

    Ok(())
}

use super::super::cite::*;

use {kutil_cli::debug::*, std::io};

/// Source tag for a [CitableFields].
pub fn source<CitableFieldsT, WriteT>(
    citable_fields: &CitableFieldsT,
    name: &str,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    CitableFieldsT: CitableFields,
    WriteT: io::Write,
{
    if let Some(citation) = citable_fields.get_field_citation(name) {
        if let Some(source) = &citation.source {
            return write!(writer, " {} {}", context.theme.delimiter("@"), context.theme.meta(source));
        }
    }
    Ok(())
}

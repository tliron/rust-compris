use super::super::cite::*;

use {kutil_cli::debug::*, owo_colors::*, std::io};

/// Source tag for a [CitableFields].
pub fn source<CitableFieldsT, WriteT>(
    citable_fields: &CitableFieldsT,
    name: &str,
    writer: &mut WriteT,
    _prefix: &DebugPrefix,
    theme: &Theme,
) -> io::Result<()>
where
    CitableFieldsT: CitableFields,
    WriteT: io::Write,
{
    if let Some(citation) = citable_fields.get_field_citation(name) {
        if let Some(source) = &citation.source {
            return write!(writer, " {} {}", "@".style(theme.delimiter), source.style(theme.meta));
        }
    }
    Ok(())
}

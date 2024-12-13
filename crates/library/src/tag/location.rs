use super::super::cite::*;

use {kutil_cli::debug::*, owo_colors::*, std::io};

/// Location tag for a [CitableFields].
pub fn location<CitableFieldsT, WriteT>(
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
        if let Some(meta) = &citation.meta {
            if let Some(location) = &meta.location {
                return write!(writer, " {} {}", "@".style(theme.delimiter), location.style(theme.meta));
            }
        }
    }
    Ok(())
}

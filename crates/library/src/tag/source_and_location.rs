use super::super::cite::*;

use {kutil_cli::debug::*, owo_colors::*, std::io};

/// Source and location tag for a [CitableFields].
pub fn source_and_location<CitableFieldsT, WriteT>(
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
        if citation.source.is_some() || citation.meta.is_some() {
            write!(writer, " {} ", "@".style(theme.delimiter))?;
        }

        if let Some(source) = &citation.source {
            write!(writer, "{}", source.style(theme.meta))?;
        }

        if let Some(meta) = &citation.meta {
            if let Some(location) = &meta.location {
                if citation.source.is_some() {
                    write!(writer, " ")?;
                }
                return write!(writer, "{}", location.style(theme.meta));
            }
        }
    }
    Ok(())
}
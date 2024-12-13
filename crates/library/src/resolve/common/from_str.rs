use super::super::{super::normal::*, cite::*, context::*, error::*, result::*};

use {
    kutil_std::error::*,
    std::{fmt, str::*},
};

/// Resolve a value into a [FromStr].
pub fn resolve_from_str<FromStrT, ContextT, ErrorT, ErrorRecipientT>(
    value: &Value,
    context: Option<&ContextT>,
    ancestor: Option<&Value>,
    errors: &mut ErrorRecipientT,
) -> ResolveResult<FromStrT, ErrorT>
where
    FromStrT: FromStr,
    FromStrT::Err: fmt::Display,
    ContextT: ResolveContext,
    ErrorT: ResolveError,
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    Ok(match value {
        Value::Text(text) => match text.value.parse() {
            Ok(parsed) => Some(parsed),

            Err(error) => {
                errors.give(
                    MalformedError::new(&tynm::type_name::<FromStrT>(), &error.to_string())
                        .with_citation_for(value, context, ancestor),
                )?;
                None
            }
        },

        _ => {
            errors
                .give(IncompatibleValueTypeError::new(value, &["text"]).with_citation_for(value, context, ancestor))?;
            None
        }
    })
}

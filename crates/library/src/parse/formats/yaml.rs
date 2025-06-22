use super::super::{
    super::{
        meta::*,
        normal::{Value, *},
    },
    Parser,
    builder::*,
    *,
};

use {
    kutil_io::reader::*,
    saphyr_parser::{Event, Parser as SaphyrParser, Span, *},
    std::{borrow::*, io},
};

impl Parser {
    /// Parses from YAML into a normal value.
    ///
    /// Is affected by [Parser::try_unsigned_integers](super::super::Parser),
    /// [Parser::allow_legacy_words](super::super::Parser),
    /// and [Parser::allow_legacy_types](super::super::Parser).
    pub fn parse_yaml<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ParseError>
    where
        ReadT: io::Read,
    {
        // https://github.com/saphyr-rs/saphyr/issues/17
        // https://github.com/saphyr-rs/saphyr/issues/16

        let mut receiver =
            YamlReceiver::new(self.try_unsigned_integers, self.allow_legacy_words, self.allow_legacy_types);
        SaphyrParser::new_from_iter(io::BufReader::new(reader).chars()).load(&mut receiver, false)?;
        receiver.value()
    }
}

const YAML_TAG_PREFIX: &'static str = "tag:yaml.org,2002:";

//
// YamlReceiver
//

/// Saphyr receiver for normal values.
struct YamlReceiver {
    try_unsigned_integers: bool,
    allow_legacy_words: bool,
    allow_legacy_types: bool,

    value_builder: ValueBuilder,
    last_span: Option<Span>,
    error: Option<ParseError>,
}

impl YamlReceiver {
    /// Constructor.
    fn new(allow_unsigned_integers: bool, allow_legacy_words: bool, allow_legacy_types: bool) -> Self {
        Self {
            try_unsigned_integers: allow_unsigned_integers,
            allow_legacy_words,
            allow_legacy_types,
            value_builder: ValueBuilder::new(),
            last_span: None,
            error: None,
        }
    }

    /// Returns the final built value.
    fn value(&mut self) -> Result<Value, ParseError> {
        match self.error.take() {
            None => Ok(self.value_builder.value()),
            Some(error) => Err(error),
        }
    }

    fn parse_yaml_tagged_scalar(
        &self,
        value: Cow<'_, str>,
        tag_prefix: &str,
        tag_suffix: &str,
        location: Location,
    ) -> Result<Value, ParseError> {
        // Check for standard schema tags
        if tag_prefix == YAML_TAG_PREFIX {
            match tag_suffix {
                // Failsafe schema, https://yaml.org/spec/1.2.2/#10113-generic-string
                "str" => {
                    return Ok(Text::from(value).with_location(Some(location)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10211-null
                "null" => {
                    self.parse_yaml_null(&value, &location)?;
                    return Ok(Null::new().with_location(Some(location)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10212-boolean
                "bool" => {
                    return Ok(Boolean::new(self.parse_yaml_bool(&value, &location)?)
                        .with_location(Some(location))
                        .into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10213-integer
                "int" => {
                    return Ok(Integer::new(Self::parse_yaml_integer(&value, &location)?)
                        .with_location(Some(location))
                        .into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10214-floating-point
                "float" => {
                    return Ok(Float::from(Self::parse_yaml_float(&value, &location)?)
                        .with_location(Some(location))
                        .into());
                }

                "binary" => {
                    // https://yaml.org/type/binary.html
                    if self.allow_legacy_types {
                        return Ok(Blob::new_from_base64(value.as_ref())?.with_location(Some(location)).into());
                    } else {
                        tracing::trace!("unsupported legacy tag suffix: {}{}", tag_prefix, tag_suffix);
                    }
                }

                _ => {
                    tracing::trace!("unsupported tag suffix: {}{}", tag_prefix, tag_suffix);
                }
            }
        } else {
            tracing::trace!("unsupported tag prefix: {}{}", tag_prefix, tag_suffix);
        }

        // Silently treat unsupported tag prefixes as strings
        Ok(Text::from(value).with_location(Some(location)).into())
    }

    fn parse_yaml_bare_scalar(&self, value: Cow<'_, str>, location: Location) -> Result<Value, ParseError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        if let Ok(_) = self.parse_yaml_null(&value, &location) {
            Ok(Null::new().with_location(Some(location)).into())
        } else if let Ok(boolean) = self.parse_yaml_bool(&value, &location) {
            Ok(Boolean::new(boolean).with_location(Some(location)).into())
        } else if let Some(unsigned_integer) =
            if self.try_unsigned_integers { Self::parse_yaml_unsigned_integer(&value) } else { None }
        {
            Ok(UnsignedInteger::new(unsigned_integer).with_location(Some(location)).into())
        } else if let Ok(integer) = Self::parse_yaml_integer(&value, &location) {
            Ok(Integer::new(integer).with_location(Some(location)).into())
        } else if let Ok(float) = Self::parse_yaml_float(&value, &location) {
            Ok(Float::from(float).with_location(Some(location)).into())
        } else {
            Ok(Text::from(value).with_location(Some(location)).into())
        }
    }

    fn parse_yaml_null(&self, value: &str, location: &Location) -> Result<(), ParseError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/null.html
            match value {
                "~" | "null" | "Null" | "NULL" => Ok(()),
                _ => Err(ScanError::new_str(location.into(), "not a null").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.1 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "null" | "Null" | "NULL" | "~" => Ok(()),
                _ => Err(ScanError::new_str(location.into(), "not a null").into()),
            }
        }
    }

    fn parse_yaml_bool(&self, value: &str, location: &Location) -> Result<bool, ParseError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/bool.html
            match value {
                "y" | "Y" | "yes" | "Yes" | "YES" | "true" | "True" | "TRUE" | "on" | "On" | "ON" => Ok(true),
                "n" | "N" | "no" | "No" | "NO" | "false" | "False" | "FALSE" | "off" | "Off" | "OFF" => Ok(false),
                _ => Err(ScanError::new_str(location.into(), "not a bool").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.2 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "true" | "True" | "TRUE" => Ok(true),
                "false" | "False" | "FALSE" => Ok(false),
                _ => Err(ScanError::new_str(location.into(), "not a bool").into()),
            }
        }
    }

    fn parse_yaml_integer(value: &str, location: &Location) -> Result<i64, ParseError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        // Section 10.2.1.3 in https://yaml.org/spec/1.2.2/#1021-tags
        if let Some(integer) = value.strip_prefix("0x") {
            if let Ok(integer) = i64::from_str_radix(integer, 16) {
                return Ok(integer);
            }
        } else if let Some(integer) = value.strip_prefix("0o") {
            if let Ok(integer) = i64::from_str_radix(integer, 8) {
                return Ok(integer);
            }
        }

        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(ScanError::new_str(location.into(), "not an integer").into()),
        }
    }

    fn parse_yaml_unsigned_integer(value: &str) -> Option<u64> {
        if let Some(unsigned_integer) = value.strip_prefix("0x") {
            if let Ok(unsigned_integer) = u64::from_str_radix(unsigned_integer, 16) {
                return Some(unsigned_integer);
            }
        } else if let Some(unsigned_integer) = value.strip_prefix("0o") {
            if let Ok(unsigned_integer) = u64::from_str_radix(unsigned_integer, 8) {
                return Some(unsigned_integer);
            }
        }

        match value.parse() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    fn parse_yaml_float(value: &str, location: &Location) -> Result<f64, ParseError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        // Section 10.2.1.4 in https://yaml.org/spec/1.2.2/#1021-tags
        match value {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Ok(f64::INFINITY),
            "-.inf" | "-.Inf" | "-.INF" => Ok(f64::NEG_INFINITY),
            ".nan" | "NaN" | ".NAN" => Ok(f64::NAN),
            _ => match value.parse() {
                Ok(value) => Ok(value),
                _ => Err(ScanError::new_str(location.into(), "not a float").into()),
            },
        }
    }
}

impl<'own, 'input> SpannedEventReceiver<'input> for YamlReceiver {
    fn on_event(&mut self, event: Event, span: Span) {
        tracing::trace!("{:?} {:?}", event, span);

        match event {
            Event::SequenceStart(anchor_id, _tag) => {
                let span = self.last_span.unwrap_or_else(|| span);
                self.value_builder.start_list_with_location(Some(span.into()), anchor(anchor_id));
            }

            Event::SequenceEnd => {
                self.value_builder.end_container();
            }

            Event::MappingStart(anchor_id, _tag) => {
                let span = self.last_span.unwrap_or_else(|| span);
                self.value_builder.start_map_with_location(Some(span.into()), anchor(anchor_id));
            }

            Event::MappingEnd => {
                self.value_builder.end_container();
            }

            Event::Scalar(value, style, anchor_id, tag) => {
                if style != ScalarStyle::Plain {
                    // All non-plain scalars are strings
                    self.value_builder.add(Text::from(value).with_location(Some(span.into())), anchor(anchor_id));
                } else {
                    // Tagged plain scalar?
                    if let Some(tag) = tag {
                        match self.parse_yaml_tagged_scalar(value, &tag.handle, &tag.suffix, span.into()) {
                            Ok(value) => self.value_builder.add(value, anchor(anchor_id)),
                            Err(error) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(error.into());
                                self.value_builder.add(Value::Nothing, anchor(anchor_id));
                            }
                        }
                    } else {
                        // Plain and untagged scalar, so determine type heuristically
                        match self.parse_yaml_bare_scalar(value, span.into()) {
                            Ok(value) => self.value_builder.add(value, anchor(anchor_id)),
                            Err(error) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(error.into());
                                self.value_builder.add(Value::Nothing, anchor(anchor_id));
                            }
                        }
                    }
                }
            }

            Event::Alias(anchor_id) => {
                if let Err(error) = self.value_builder.add_referenced(anchor_id) {
                    self.error = Some(error);
                    self.value_builder.add(Value::Nothing, None);
                }
            }

            _ => {}
        }

        // TODO: always?
        self.last_span = Some(span);
    }
}

impl From<Span> for Location {
    fn from(span: Span) -> Self {
        // Saphyr seems to have the first line at 1, but the first column at 0
        let line = match span.start.line() {
            0 => 0,
            line => line - 1,
        };
        Location::new(span.start.index(), line, span.start.col())
    }
}

// Yeah, it's inefficient to convert Location back to a Marker,
// but we only ever do this for errors
impl From<&Location> for Marker {
    fn from(location: &Location) -> Self {
        let (row, column) = {
            match location.row_and_column {
                // Saphyr seems to have the first line at 1, but the first column at 0
                Some((row, column)) => match column {
                    Some(column) => (row + 1, column),
                    None => (row, 0),
                },
                None => (0, 0),
            }
        };

        Marker::new(location.index.unwrap_or(0), row, column)
    }
}

fn anchor(anchor_id: usize) -> Option<usize> {
    if anchor_id != 0 { Some(anchor_id) } else { None }
}

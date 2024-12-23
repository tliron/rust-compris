use super::super::{
    super::normal::{Value, *},
    builder::*,
    *,
};

use {
    saphyr_parser::{Event, Span, *},
    std::io::Read,
    tracing::*,
};

//
// Reader
//

impl Reader {
    /// Reads from YAML into a normal value.
    ///
    /// Is affected by [Reader::try_unsigned_integers](super::super::Reader),
    /// [Reader::allow_legacy_words](super::super::Reader),
    /// and [Reader::allow_legacy_types](super::super::Reader).
    pub fn read_yaml<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
        // https://github.com/saphyr-rs/saphyr/issues/17
        // https://github.com/saphyr-rs/saphyr/issues/16

        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let mut receiver =
            YamlReceiver::new(self.try_unsigned_integers, self.allow_legacy_words, self.allow_legacy_types);
        Parser::new_from_str(&string).load(&mut receiver, false)?;
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
    error: Option<ReadError>,
}

impl YamlReceiver {
    /// Constructor.
    fn new(allow_unsigned_integers: bool, allow_legacy_words: bool, allow_legacy_types: bool) -> Self {
        Self {
            try_unsigned_integers: allow_unsigned_integers,
            allow_legacy_words,
            allow_legacy_types,
            value_builder: ValueBuilder::new(),
            error: None,
        }
    }

    /// Returns the final built value.
    fn value(&mut self) -> Result<Value, ReadError> {
        match self.error.take() {
            Some(err) => Err(err),
            None => Ok(self.value_builder.value()),
        }
    }

    fn parse_yaml_tagged_scalar(
        &self,
        value: String,
        tag_prefix: &str,
        tag_suffix: &str,
        coordinates: Coordinates,
    ) -> Result<Value, ReadError> {
        // Check for standard schema tags
        if tag_prefix == YAML_TAG_PREFIX {
            match tag_suffix {
                // Failsafe schema, https://yaml.org/spec/1.2.2/#10113-generic-string
                "str" => {
                    return Ok(Text::new(value).with_coordinates(Some(coordinates)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10211-null
                "null" => {
                    self.parse_yaml_null(&value, &coordinates)?;
                    return Ok(Null::new().with_coordinates(Some(coordinates)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10212-boolean
                "bool" => {
                    return Ok(Boolean::new(self.parse_yaml_bool(&value, &coordinates)?)
                        .with_coordinates(Some(coordinates))
                        .into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10213-integer
                "int" => {
                    return Ok(Integer::new(Self::parse_yaml_integer(&value, &coordinates)?)
                        .with_coordinates(Some(coordinates))
                        .into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10214-floating-point
                "float" => {
                    return Ok(Float::new(Self::parse_yaml_float(&value, &coordinates)?)
                        .with_coordinates(Some(coordinates))
                        .into());
                }

                "binary" => {
                    // https://yaml.org/type/binary.html
                    if self.allow_legacy_types {
                        return Ok(Bytes::new_from_base64(&value)?.with_coordinates(Some(coordinates)).into());
                    } else {
                        trace!("unsupported legacy tag suffix: {}{}", tag_prefix, tag_suffix);
                    }
                }

                _ => {
                    trace!("unsupported tag suffix: {}{}", tag_prefix, tag_suffix);
                }
            }
        } else {
            trace!("unsupported tag prefix: {}{}", tag_prefix, tag_suffix);
        }

        // Silently treat unsupported tag prefixes as strings
        Ok(Text::new(value).with_coordinates(Some(coordinates)).into())
    }

    fn parse_yaml_bare_scalar(&self, value: String, coordinates: Coordinates) -> Result<Value, ReadError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        if let Ok(_) = self.parse_yaml_null(&value, &coordinates) {
            Ok(Null::new().with_coordinates(Some(coordinates)).into())
        } else if let Ok(boolean) = self.parse_yaml_bool(&value, &coordinates) {
            Ok(Boolean::new(boolean).with_coordinates(Some(coordinates)).into())
        } else if let Some(unsigned_integer) =
            if self.try_unsigned_integers { Self::parse_yaml_unsigned_integer(&value) } else { None }
        {
            Ok(UnsignedInteger::new(unsigned_integer).with_coordinates(Some(coordinates)).into())
        } else if let Ok(integer) = Self::parse_yaml_integer(&value, &coordinates) {
            Ok(Integer::new(integer).with_coordinates(Some(coordinates)).into())
        } else if let Ok(float) = Self::parse_yaml_float(&value, &coordinates) {
            Ok(Float::new(float).with_coordinates(Some(coordinates)).into())
        } else {
            Ok(Text::new(value).with_coordinates(Some(coordinates)).into())
        }
    }

    fn parse_yaml_null(&self, value: &str, coordinates: &Coordinates) -> Result<(), ReadError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/null.html
            match value {
                "~" | "null" | "Null" | "NULL" => Ok(()),
                _ => Err(ScanError::new_str(coordinates.into(), "not a null").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.1 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "null" | "Null" | "NULL" | "~" => Ok(()),
                _ => Err(ScanError::new_str(coordinates.into(), "not a null").into()),
            }
        }
    }

    fn parse_yaml_bool(&self, value: &str, coordinates: &Coordinates) -> Result<bool, ReadError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/bool.html
            match value {
                "y" | "Y" | "yes" | "Yes" | "YES" | "true" | "True" | "TRUE" | "on" | "On" | "ON" => Ok(true),
                "n" | "N" | "no" | "No" | "NO" | "false" | "False" | "FALSE" | "off" | "Off" | "OFF" => Ok(false),
                _ => Err(ScanError::new_str(coordinates.into(), "not a bool").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.2 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "true" | "True" | "TRUE" => Ok(true),
                "false" | "False" | "FALSE" => Ok(false),
                _ => Err(ScanError::new_str(coordinates.into(), "not a bool").into()),
            }
        }
    }

    fn parse_yaml_integer(value: &str, coordinates: &Coordinates) -> Result<i64, ReadError> {
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
            Err(_) => Err(ScanError::new_str(coordinates.into(), "not an integer").into()),
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

    fn parse_yaml_float(value: &str, coordinates: &Coordinates) -> Result<f64, ReadError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        // Section 10.2.1.4 in https://yaml.org/spec/1.2.2/#1021-tags
        match value {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Ok(f64::INFINITY),
            "-.inf" | "-.Inf" | "-.INF" => Ok(f64::NEG_INFINITY),
            ".nan" | "NaN" | ".NAN" => Ok(f64::NAN),
            _ => match value.parse() {
                Ok(value) => Ok(value),
                _ => Err(ScanError::new_str(coordinates.into(), "not a float").into()),
            },
        }
    }
}

impl<'a> SpannedEventReceiver for YamlReceiver {
    fn on_event(&mut self, event: Event, span: Span) {
        trace!("{:?} {:?}", event, span);

        match event {
            Event::SequenceStart(_anchor_id, _tag) => {
                self.value_builder.start_list_with_coordinates(Some(span.into()));
            }

            Event::SequenceEnd => {
                self.value_builder.end_container();
            }

            Event::MappingStart(_anchor_id, _tag) => {
                self.value_builder.start_map_with_coordinates(Some(span.into()));
            }

            Event::MappingEnd => {
                self.value_builder.end_container();
            }

            Event::Scalar(value, style, _anchor_id, tag) => {
                if style != TScalarStyle::Plain {
                    // All non-plain scalars are strings
                    self.value_builder.add(Text::new(value).with_coordinates(Some(span.into())));
                } else {
                    // Tagged plain scalar?
                    if let Some(Tag { ref handle, ref suffix }) = tag {
                        match self.parse_yaml_tagged_scalar(value, handle, suffix, span.into()) {
                            Ok(value) => self.value_builder.add(value),
                            Err(err) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(err.into());
                                self.value_builder.add(Value::Nothing);
                            }
                        }
                    } else {
                        // Plain and untagged scalar, so determine type heuristically
                        match self.parse_yaml_bare_scalar(value, span.into()) {
                            Ok(value) => self.value_builder.add(value),
                            Err(err) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(err.into());
                                self.value_builder.add(Value::Nothing);
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

impl From<Span> for Coordinates {
    fn from(span: Span) -> Self {
        Coordinates::new(span.start.index(), span.start.line(), span.start.col())
    }
}

// Yeah, it's inefficient to convert Coordinates back to a Marker,
// but we only ever do this for errors
impl From<&Coordinates> for Marker {
    fn from(coordinates: &Coordinates) -> Self {
        let (row, column) = match coordinates.row_and_column {
            Some((row, column)) => match column {
                Some(column) => (row, column),
                None => (row, 0),
            },
            None => (0, 0),
        };

        Marker::new(
            match coordinates.index {
                Some(index) => index,
                None => 0,
            },
            row,
            column,
        )
    }
}

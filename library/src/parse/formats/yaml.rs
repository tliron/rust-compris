use super::super::{
    super::{
        annotate::{Span, *},
        normal::{Variant, *},
    },
    Parser,
    builder::*,
    *,
};

use {
    kutil::{io::reader::*, std::immutable::*},
    saphyr_parser::{Event, Parser as SaphyrParser, Span as SaphyrSpan, *},
    std::{borrow::*, io},
};

impl Parser {
    /// Parses YAML into a [Variant].
    ///
    /// Is affected by [Parser::try_unsigned_integers](super::super::Parser),
    /// [Parser::allow_legacy_words](super::super::Parser),
    /// and [Parser::allow_legacy_types](super::super::Parser).
    pub fn parse_yaml<ReadT, AnnotatedT>(&self, reader: &mut ReadT) -> Result<Variant<AnnotatedT>, ParseError>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
    {
        // https://github.com/saphyr-rs/saphyr/issues/17
        // https://github.com/saphyr-rs/saphyr/issues/16

        let mut receiver = YamlReceiver::new(
            self.source.clone(),
            self.try_unsigned_integers,
            self.allow_legacy_words,
            self.allow_legacy_types,
        );
        SaphyrParser::new_from_iter(io::BufReader::new(reader).chars()).load(&mut receiver, false)?;
        receiver.value()
    }
}

const YAML_TAG_PREFIX: &'static str = "tag:yaml.org,2002:";

//
// YamlReceiver
//

/// Saphyr receiver for normal types.
struct YamlReceiver<AnnotatedT> {
    try_unsigned_integers: bool,
    allow_legacy_words: bool,
    allow_legacy_types: bool,

    value_builder: VariantBuilder<AnnotatedT>,
    last_span: Option<SaphyrSpan>,
    error: Option<ParseError>,

    span: fn(&SaphyrSpan) -> Option<Span>,
    collection_span: fn(&Self, &SaphyrSpan) -> Option<Span>,
}

impl<AnnotatedT> YamlReceiver<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    /// Constructor.
    fn new(
        source: Option<ByteString>,
        allow_unsigned_integers: bool,
        allow_legacy_words: bool,
        allow_legacy_types: bool,
    ) -> Self {
        Self {
            try_unsigned_integers: allow_unsigned_integers,
            allow_legacy_words,
            allow_legacy_types,
            value_builder: VariantBuilder::new(source.clone()),
            last_span: None,
            error: None,
            span: if AnnotatedT::can_have_annotations() { |span| Some(span.into()) } else { |_| None },
            collection_span: if AnnotatedT::can_have_annotations() {
                |yaml_receiver, span| Some(yaml_receiver.last_span.as_ref().unwrap_or_else(|| span).into())
            } else {
                |_, _| None
            },
        }
    }

    /// Returns the final built value.
    fn value(&mut self) -> Result<Variant<AnnotatedT>, ParseError>
    where
        AnnotatedT: Default,
    {
        match self.error.take() {
            None => Ok(self.value_builder.finalize()),
            Some(error) => Err(error),
        }
    }

    fn parse_yaml_tagged_scalar(
        &self,
        value: Cow<'_, str>,
        tag_prefix: &str,
        tag_suffix: &str,
        span: &SaphyrSpan,
    ) -> Result<Variant<AnnotatedT>, ParseError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        // Check for standard schema tags
        if tag_prefix == YAML_TAG_PREFIX {
            match tag_suffix {
                // Failsafe schema, https://yaml.org/spec/1.2.2/#10113-generic-string
                "str" => {
                    return Ok(Text::from(value).with_span((self.span)(span)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10211-null
                "null" => {
                    self.parse_yaml_null(&value, &span)?;
                    return Ok(Null::default().with_span((self.span)(span)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10212-boolean
                "bool" => {
                    return Ok(Boolean::from(self.parse_yaml_bool(&value, &span)?).with_span((self.span)(span)).into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10213-integer
                "int" => {
                    return Ok(Integer::from(Self::parse_yaml_integer(&value, &span)?)
                        .with_span((self.span)(span))
                        .into());
                }

                // JSON schema, https://yaml.org/spec/1.2.2/#10214-floating-point
                "float" => {
                    return Ok(Float::from(Self::parse_yaml_float(&value, &span)?).with_span((self.span)(span)).into());
                }

                "binary" => {
                    // https://yaml.org/type/binary.html
                    if self.allow_legacy_types {
                        return Ok(Blob::new_from_base64(value.as_ref())?.with_span((self.span)(span)).into());
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
        Ok(Text::from(value).with_span((self.span)(span)).into())
    }

    fn parse_yaml_bare_scalar(&self, value: Cow<'_, str>, span: &SaphyrSpan) -> Result<Variant<AnnotatedT>, ParseError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        if self.parse_yaml_null(&value, span).is_ok() {
            Ok(Null::default().with_span((self.span)(span)).into())
        } else if let Ok(boolean) = self.parse_yaml_bool(&value, &span) {
            Ok(Boolean::from(boolean).with_span((self.span)(span)).into())
        } else if let Some(unsigned_integer) =
            if self.try_unsigned_integers { Self::try_parse_yaml_unsigned_integer(&value) } else { None }
        {
            Ok(UnsignedInteger::from(unsigned_integer).with_span((self.span)(span)).into())
        } else if let Ok(integer) = Self::parse_yaml_integer(&value, &span) {
            Ok(Integer::from(integer).with_span((self.span)(span)).into())
        } else if let Ok(float) = Self::parse_yaml_float(&value, span) {
            Ok(Float::from(float).with_span((self.span)(span)).into())
        } else {
            Ok(Text::from(value).with_span((self.span)(span)).into())
        }
    }

    fn parse_yaml_null(&self, value: &str, span: &SaphyrSpan) -> Result<(), ParseError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/null.html
            match value {
                "~" | "null" | "Null" | "NULL" => Ok(()),
                _ => Err(ScanError::new_str(span.start, "not a null").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.1 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "null" | "Null" | "NULL" | "~" => Ok(()),
                _ => Err(ScanError::new_str(span.start, "not a null").into()),
            }
        }
    }

    fn parse_yaml_bool(&self, value: &str, span: &SaphyrSpan) -> Result<bool, ParseError> {
        if self.allow_legacy_words {
            // https://yaml.org/type/bool.html
            match value {
                "y" | "Y" | "yes" | "Yes" | "YES" | "true" | "True" | "TRUE" | "on" | "On" | "ON" => Ok(true),
                "n" | "N" | "no" | "No" | "NO" | "false" | "False" | "FALSE" | "off" | "Off" | "OFF" => Ok(false),
                _ => Err(ScanError::new_str(span.start, "not a bool").into()),
            }
        } else {
            // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
            // Section 10.2.1.2 in https://yaml.org/spec/1.2.2/#1021-tags
            match value {
                "true" | "True" | "TRUE" => Ok(true),
                "false" | "False" | "FALSE" => Ok(false),
                _ => Err(ScanError::new_str(span.start, "not a bool").into()),
            }
        }
    }

    fn parse_yaml_integer(value: &str, span: &SaphyrSpan) -> Result<i64, ParseError> {
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

        value.parse().map_err(|_| ScanError::new_str(span.start, "not an integer").into())
    }

    fn try_parse_yaml_unsigned_integer(value: &str) -> Option<u64> {
        if let Some(unsigned_integer) = value.strip_prefix("0x") {
            if let Ok(unsigned_integer) = u64::from_str_radix(unsigned_integer, 16) {
                return Some(unsigned_integer);
            }
        } else if let Some(unsigned_integer) = value.strip_prefix("0o") {
            if let Ok(unsigned_integer) = u64::from_str_radix(unsigned_integer, 8) {
                return Some(unsigned_integer);
            }
        }

        value.parse().ok()
    }

    fn parse_yaml_float(value: &str, span: &SaphyrSpan) -> Result<f64, ParseError> {
        // Core schema, https://yaml.org/spec/1.2.2/#1032-tag-resolution
        // Section 10.2.1.4 in https://yaml.org/spec/1.2.2/#1021-tags
        match value {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Ok(f64::INFINITY),
            "-.inf" | "-.Inf" | "-.INF" => Ok(f64::NEG_INFINITY),
            ".nan" | "NaN" | ".NAN" => Ok(f64::NAN),
            _ => value.parse().map_err(|_| ScanError::new_str(span.start, "not a float").into()),
        }
    }
}

impl<'own, 'input, AnnotatedT> SpannedEventReceiver<'input> for YamlReceiver<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn on_event(&mut self, event: Event, span: SaphyrSpan) {
        tracing::trace!("{:?} {:?}", event, span);

        match event {
            Event::SequenceStart(anchor_id, _tag) => {
                self.value_builder.start_list_with_span((self.collection_span)(self, &span), anchor(anchor_id));
            }

            Event::SequenceEnd => {
                self.value_builder.end_container();
            }

            Event::MappingStart(anchor_id, _tag) => {
                self.value_builder.start_map_with_span((self.collection_span)(self, &span), anchor(anchor_id));
            }

            Event::MappingEnd => {
                self.value_builder.end_container();
            }

            Event::Scalar(value, style, anchor_id, tag) => {
                if style != ScalarStyle::Plain {
                    // All non-plain scalars are strings
                    self.value_builder.add(Text::from(value).with_span((self.span)(&span)), anchor(anchor_id));
                } else {
                    // Tagged plain scalar?
                    if let Some(tag) = tag {
                        match self.parse_yaml_tagged_scalar(value, &tag.handle, &tag.suffix, &span) {
                            Ok(value) => self.value_builder.add(value, anchor(anchor_id)),
                            Err(error) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(error.into());
                                self.value_builder.add(Variant::Undefined, anchor(anchor_id));
                            }
                        }
                    } else {
                        // Plain and untagged scalar, so determine type heuristically
                        match self.parse_yaml_bare_scalar(value, &span) {
                            Ok(value) => self.value_builder.add(value, anchor(anchor_id)),
                            Err(error) => {
                                // See: https://github.com/saphyr-rs/saphyr/issues/20
                                self.error = Some(error.into());
                                self.value_builder.add(Variant::Undefined, anchor(anchor_id));
                            }
                        }
                    }
                }
            }

            Event::Alias(anchor_id) => {
                if let Err(error) = self.value_builder.add_referenced(anchor_id) {
                    self.error = Some(error);
                    self.value_builder.add(Variant::Undefined, None);
                }
            }

            _ => {}
        }

        if AnnotatedT::can_have_annotations() {
            self.last_span = Some(span);
        }
    }
}

impl From<&SaphyrSpan> for Span {
    fn from(span: &SaphyrSpan) -> Self {
        // Saphyr seems to have the first line at 1, but the first column at 0

        let start_line = match span.start.line() {
            0 => 0,
            line => line - 1,
        };

        let end_line = match span.end.line() {
            0 => 0,
            line => line - 1,
        };

        Span::new(
            Location::new(Some(span.start.index()), Some(start_line), Some(span.start.col())),
            Some(Location::new(Some(span.end.index()), Some(end_line), Some(span.end.col()))),
        )
    }
}

fn anchor(anchor_id: usize) -> Option<usize> {
    if anchor_id != 0 { Some(anchor_id) } else { None }
}

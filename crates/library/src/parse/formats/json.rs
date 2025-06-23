use super::super::{
    super::{
        annotation::*,
        hints::*,
        normal::{Value, *},
    },
    builder::*,
    *,
};

use {std::io, struson::reader::*};

impl Parser {
    /// Parses JSON into a [Value].
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_json<ReadT, AnnotationsT>(&self, reader: &mut ReadT) -> Result<Value<AnnotationsT>, ParseError>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
    {
        self.parse_json_with_hints(reader, None)
    }

    /// Parses XJSON into a [Value].
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_xjson<ReadT, AnnotationsT>(&self, reader: &mut ReadT) -> Result<Value<AnnotationsT>, ParseError>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
    {
        self.parse_json_with_hints(reader, Some(&Hints::xjson()))
    }

    /// Parses JSON into a [Value].
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_json_with_hints<ReadT, AnnotationsT>(
        &self,
        reader: &mut ReadT,
        hints: Option<&Hints>,
    ) -> Result<Value<AnnotationsT>, ParseError>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
    {
        let mut reader = JsonStreamReader::new(reader);
        let mut value_builder = ValueBuilder::new(self.source.clone());
        read_next_json(&mut reader, &mut value_builder, hints, self.try_integers, self.try_unsigned_integers)?;
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_json<JsonReaderT, AnnotationsT>(
    reader: &mut JsonReaderT,
    value_builder: &mut ValueBuilder<AnnotationsT>,
    hints: Option<&Hints>,
    try_integers: bool,
    try_unsigned_integers: bool,
) -> Result<(), ParseError>
where
    JsonReaderT: JsonReader,
    AnnotationsT: Annotated + Clone + Default,
{
    let get_span = if AnnotationsT::is_annotated() {
        |reader: &mut JsonReaderT| -> Option<Span> { get_json_span(reader) }
    } else {
        |_reader: &mut JsonReaderT| -> Option<Span> { None }
    };

    let value = reader.peek()?;
    tracing::trace!("{}", value);
    match value {
        ValueType::Null => {
            let span = get_span(reader);
            reader.next_null()?;
            value_builder.add(Null::default().with_span(span), None);
        }

        ValueType::Number => {
            if try_integers || try_unsigned_integers {
                let span = get_span(reader);
                let number = reader.next_number_as_str()?;
                if let Some(number) = if try_unsigned_integers { number.parse::<u64>().ok() } else { None } {
                    value_builder.add(UnsignedInteger::new(number).with_span(span), None);
                } else if let Some(number) = if try_integers { number.parse::<i64>().ok() } else { None } {
                    value_builder.add(Integer::new(number).with_span(span), None);
                } else {
                    value_builder.add(Float::from(number.parse::<f64>()?).with_span(span), None);
                }
            } else {
                let span = get_span(reader);
                let number: f64 = reader.next_number()??;
                value_builder.add(Float::from(number).with_span(span), None);
            }
        }

        ValueType::Boolean => {
            let span = get_span(reader);
            value_builder.add(Boolean::new(reader.next_bool()?).with_span(span), None);
        }

        ValueType::String => {
            let span = get_span(reader);
            value_builder.add(Text::from(reader.next_str()?).with_span(span), None);
        }

        ValueType::Array => {
            let span = get_span(reader);
            reader.begin_array()?;
            value_builder.start_list_with_span(span, None);
            while reader.has_next()? {
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container();
            reader.end_array()?;
        }

        ValueType::Object => {
            let span = get_span(reader);
            reader.begin_object()?;
            value_builder.start_map_with_span(span, None);
            while reader.has_next()? {
                // Key
                let span = get_span(reader);
                value_builder.add(Text::from(reader.next_name()?).with_span(span), None);

                // Value
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container_with_hints(hints)?;
            reader.end_object()?;
        }
    }

    Ok(())
}

// Note that Struson only provides the start of the span
fn get_json_span(reader: &mut impl JsonReader) -> Option<Span> {
    let mut span = Span::default();
    let mut some = false;

    let position = reader.current_position(false);

    if let Some(data_pos) = position.data_pos {
        some = true;
        span.start.index = Some(data_pos as usize);
    }

    if let Some(line_pos) = position.line_pos {
        some = true;
        span.start.row = Some(line_pos.line as usize);
        span.start.column = Some(line_pos.column as usize);
    };

    if some { Some(span) } else { None }
}

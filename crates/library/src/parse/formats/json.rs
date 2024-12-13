use super::super::{
    super::{
        hints::*,
        meta::*,
        normal::{Value, *},
    },
    builder::*,
    *,
};

use {std::io, struson::reader::*, tracing::*};

impl Parser {
    /// Parses from JSON into a normal value.
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_json<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ParseError>
    where
        ReadT: io::Read,
    {
        self.parse_json_with_hints(reader, None)
    }

    /// Parses from XJSON into a normal value.
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_xjson<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ParseError>
    where
        ReadT: io::Read,
    {
        self.parse_json_with_hints(reader, Some(&Hints::xjson()))
    }

    /// Parses from JSON into a normal value.
    ///
    /// Is affected by [Parser::try_integers](super::super::Parser)
    /// and [Parser::try_unsigned_integers](super::super::Parser).
    pub fn parse_json_with_hints<ReadT>(&self, reader: &mut ReadT, hints: Option<&Hints>) -> Result<Value, ParseError>
    where
        ReadT: io::Read,
    {
        let mut reader = JsonStreamReader::new(reader);
        let mut value_builder = ValueBuilder::new();
        read_next_json(&mut reader, &mut value_builder, hints, self.try_integers, self.try_unsigned_integers)?;
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_json<JsonReaderT>(
    reader: &mut JsonReaderT,
    value_builder: &mut ValueBuilder,
    hints: Option<&Hints>,
    try_integers: bool,
    try_unsigned_integers: bool,
) -> Result<(), ParseError>
where
    JsonReaderT: JsonReader,
{
    let value = reader.peek()?;
    trace!("{}", value);
    match value {
        ValueType::Null => {
            let location = get_json_location(reader);
            reader.next_null()?;
            value_builder.add(Null::new().with_location(location));
        }

        ValueType::Number => {
            if try_integers || try_unsigned_integers {
                let location = get_json_location(reader);
                let number = reader.next_number_as_str()?;
                if let Some(number) = if try_unsigned_integers { number.parse::<u64>().ok() } else { None } {
                    value_builder.add(UnsignedInteger::new(number).with_location(location));
                } else if let Some(number) = if try_integers { number.parse::<i64>().ok() } else { None } {
                    value_builder.add(Integer::new(number).with_location(location));
                } else {
                    value_builder.add(Float::new(number.parse::<f64>()?).with_location(location));
                }
            } else {
                let location = get_json_location(reader);
                let number: f64 = reader.next_number()??;
                value_builder.add(Float::new(number).with_location(location));
            }
        }

        ValueType::Boolean => {
            let location = get_json_location(reader);
            value_builder.add(Boolean::new(reader.next_bool()?).with_location(location));
        }

        ValueType::String => {
            let location = get_json_location(reader);
            value_builder.add(Text::new(reader.next_string()?).with_location(location));
        }

        ValueType::Array => {
            let location = get_json_location(reader);
            reader.begin_array()?;
            value_builder.start_list_with_location(location);
            while reader.has_next()? {
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container();
            reader.end_array()?;
        }

        ValueType::Object => {
            let location = get_json_location(reader);
            reader.begin_object()?;
            value_builder.start_map_with_location(location);
            while reader.has_next()? {
                // Key
                let location = get_json_location(reader);
                value_builder.add(Text::new(reader.next_name_owned()?).with_location(location));

                // Value
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container_with_hints(hints)?;
            reader.end_object()?;
        }
    }

    Ok(())
}

fn get_json_location(reader: &mut impl JsonReader) -> Option<Location> {
    let mut location = Location::default();
    let mut some = false;

    let position = reader.current_position(false);

    if let Some(data_pos) = position.data_pos {
        some = true;
        location.index = Some(data_pos as usize);
    }

    if let Some(line_pos) = position.line_pos {
        some = true;
        location.row_and_column = Some((line_pos.line as usize, Some(line_pos.column as usize)));
    };

    if some {
        Some(location)
    } else {
        None
    }
}

use super::super::{
    super::{Value, *},
    value_builder::*,
    *,
};

use {
    std::{io::*, result::Result},
    struson::reader::*,
    tracing::*,
};

impl Reader {
    /// Reads from JSON into a normal value.
    ///
    /// Is affected by [Reader::try_integers] and [Reader::try_unsigned_integers].
    pub fn read_json<R: Read>(&self, reader: &mut R) -> Result<Value, ReadError> {
        self.read_json_with_hints(reader, None)
    }

    /// Reads from XJSON into a normal value.
    ///
    /// Is affected by [Reader::try_integers] and [Reader::try_unsigned_integers].
    pub fn read_xjson<R: Read>(&self, reader: &mut R) -> Result<Value, ReadError> {
        self.read_json_with_hints(reader, Some(&Hints::xjson()))
    }

    fn read_json_with_hints<R: Read>(&self, reader: &mut R, hints: Option<&Hints>) -> Result<Value, ReadError> {
        let mut reader = JsonStreamReader::new(reader);
        let mut value_builder = ValueBuilder::new();
        read_next_json(&mut reader, &mut value_builder, hints, self.try_integers, self.try_unsigned_integers)?;
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_json(
    reader: &mut impl JsonReader,
    value_builder: &mut ValueBuilder,
    hints: Option<&Hints>,
    try_integers: bool,
    try_unsigned_integers: bool,
) -> Result<(), ReadError> {
    let value = reader.peek()?;
    trace!("{}", value);
    match value {
        ValueType::Null => {
            reader.next_null()?;
            value_builder.add(Null::new().with_location(get_json_location(reader)));
        }

        ValueType::Number => {
            if try_integers || try_unsigned_integers {
                let number = reader.next_number_as_str()?;
                if let Some(number) = if try_unsigned_integers { number.parse::<u64>().ok() } else { None } {
                    value_builder.add(UnsignedInteger::new(number).with_location(get_json_location(reader)));
                } else if let Some(number) = if try_integers { number.parse::<i64>().ok() } else { None } {
                    value_builder.add(Integer::new(number).with_location(get_json_location(reader)));
                } else {
                    value_builder.add(Float::new(number.parse::<f64>()?).with_location(get_json_location(reader)));
                }
            } else {
                let number: f64 = reader.next_number()??;
                value_builder.add(Float::new(number).with_location(get_json_location(reader)));
            }
        }

        ValueType::Boolean => {
            value_builder.add(Boolean::new(reader.next_bool()?).with_location(get_json_location(reader)));
        }

        ValueType::String => {
            value_builder.add(Text::new(reader.next_string()?).with_location(get_json_location(reader)));
        }

        ValueType::Array => {
            reader.begin_array()?;
            value_builder.start_list_with_location(get_json_location(reader));
            while reader.has_next()? {
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container();
            reader.end_array()?;
        }

        ValueType::Object => {
            reader.begin_object()?;
            value_builder.start_map_with_location(get_json_location(reader));
            while reader.has_next()? {
                // Key
                value_builder.add(Text::new(reader.next_name_owned()?).with_location(get_json_location(reader)));

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
        location.row_and_column = Some((line_pos.line as usize, line_pos.column as usize));
    };

    if some {
        Some(location)
    } else {
        None
    }
}

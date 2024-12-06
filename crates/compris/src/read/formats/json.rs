use super::super::{super::*, errors::*, reader::*, value_builder::*};

use {std::io::Read, struson::reader::*, tracing::trace};

impl<R: Read> Reader<R> {
    pub fn read_json(&mut self) -> Result<Value, ReadError> {
        self.read_json_for_format(false)
    }

    pub fn read_xjson(&mut self) -> Result<Value, ReadError> {
        self.read_json_for_format(true)
    }

    fn read_json_for_format(&mut self, xjson: bool) -> Result<Value, ReadError> {
        let mut reader = JsonStreamReader::new(self.reader.by_ref());
        let mut value_builder = ValueBuilder::new();
        read_next_json(&mut reader, &mut value_builder, xjson, self.allow_integers, self.allow_unsigned_integers)?;
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_json(
    reader: &mut impl JsonReader,
    value_builder: &mut ValueBuilder,
    xjson: bool,
    allow_integers: bool,
    allow_unsigned_integers: bool,
) -> Result<(), ReadError> {
    let value = reader.peek()?;
    trace!("{}", value);
    match value {
        ValueType::Null => {
            reader.next_null()?;
            value_builder.add(Null::new().with_location_option(get_json_location(reader)));
        }

        ValueType::Number => {
            if allow_integers || allow_unsigned_integers {
                let number = reader.next_number_as_str()?;
                if let Some(number) = if allow_unsigned_integers { number.parse().ok() } else { None } {
                    value_builder.add(UnsignedInteger::new(number).with_location_option(get_json_location(reader)));
                } else if let Some(number) = if allow_integers { number.parse().ok() } else { None } {
                    value_builder.add(Integer::new(number).with_location_option(get_json_location(reader)));
                } else {
                    value_builder.add(Float::new(number.parse()?).with_location_option(get_json_location(reader)));
                }
            } else {
                let number = reader.next_number()??;
                value_builder.add(Float::new(number).with_location_option(get_json_location(reader)));
            }
        }

        ValueType::Boolean => {
            value_builder.add(Boolean::new(reader.next_bool()?).with_location_option(get_json_location(reader)));
        }

        ValueType::String => {
            value_builder.add(String::new(reader.next_string()?).with_location_option(get_json_location(reader)));
        }

        ValueType::Array => {
            reader.begin_array()?;
            value_builder.start_list();
            while reader.has_next()? {
                read_next_json(reader, value_builder, xjson, allow_integers, allow_unsigned_integers)?;
            }
            value_builder.end_container();
            reader.end_array()?;
        }

        ValueType::Object => {
            reader.begin_object()?;
            value_builder.start_map();
            while reader.has_next()? {
                // Key
                value_builder
                    .add(String::new(reader.next_name_owned()?).with_location_option(get_json_location(reader)));

                // Value
                read_next_json(reader, value_builder, xjson, allow_integers, allow_unsigned_integers)?;
            }
            value_builder.end_container_with_hints(xjson)?;
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
        location.index = data_pos as usize;
    }

    if let Some(line_pos) = position.line_pos {
        some = true;
        location.row = line_pos.line as usize;
        location.column = line_pos.column as usize;
    };

    if some {
        Some(location)
    } else {
        None
    }
}

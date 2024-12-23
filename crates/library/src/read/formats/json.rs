use super::super::{
    super::{
        hints::*,
        normal::{Value, *},
    },
    builder::*,
    *,
};

use {std::io::Read, struson::reader::*, tracing::*};

//
// Reader
//

impl Reader {
    /// Reads from JSON into a normal value.
    ///
    /// Is affected by [Reader::try_integers](super::super::Reader)
    /// and [Reader::try_unsigned_integers](super::super::Reader).
    pub fn read_json<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
        self.read_json_with_hints(reader, None)
    }

    /// Reads from XJSON into a normal value.
    ///
    /// Is affected by [Reader::try_integers](super::super::Reader)
    /// and [Reader::try_unsigned_integers](super::super::Reader).
    pub fn read_xjson<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
        self.read_json_with_hints(reader, Some(&Hints::xjson()))
    }

    /// Reads from JSON into a normal value.
    ///
    /// Is affected by [Reader::try_integers](super::super::Reader)
    /// and [Reader::try_unsigned_integers](super::super::Reader).
    pub fn read_json_with_hints<ReadT>(&self, reader: &mut ReadT, hints: Option<&Hints>) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
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
            value_builder.add(Null::new().with_coordinates(get_json_coordinates(reader)));
        }

        ValueType::Number => {
            if try_integers || try_unsigned_integers {
                let number = reader.next_number_as_str()?;
                if let Some(number) = if try_unsigned_integers { number.parse::<u64>().ok() } else { None } {
                    value_builder.add(UnsignedInteger::new(number).with_coordinates(get_json_coordinates(reader)));
                } else if let Some(number) = if try_integers { number.parse::<i64>().ok() } else { None } {
                    value_builder.add(Integer::new(number).with_coordinates(get_json_coordinates(reader)));
                } else {
                    value_builder
                        .add(Float::new(number.parse::<f64>()?).with_coordinates(get_json_coordinates(reader)));
                }
            } else {
                let number: f64 = reader.next_number()??;
                value_builder.add(Float::new(number).with_coordinates(get_json_coordinates(reader)));
            }
        }

        ValueType::Boolean => {
            value_builder.add(Boolean::new(reader.next_bool()?).with_coordinates(get_json_coordinates(reader)));
        }

        ValueType::String => {
            value_builder.add(Text::new(reader.next_string()?).with_coordinates(get_json_coordinates(reader)));
        }

        ValueType::Array => {
            reader.begin_array()?;
            value_builder.start_list_with_coordinates(get_json_coordinates(reader));
            while reader.has_next()? {
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container();
            reader.end_array()?;
        }

        ValueType::Object => {
            reader.begin_object()?;
            value_builder.start_map_with_coordinates(get_json_coordinates(reader));
            while reader.has_next()? {
                // Key
                value_builder.add(Text::new(reader.next_name_owned()?).with_coordinates(get_json_coordinates(reader)));

                // Value
                read_next_json(reader, value_builder, hints, try_integers, try_unsigned_integers)?;
            }
            value_builder.end_container_with_hints(hints)?;
            reader.end_object()?;
        }
    }

    Ok(())
}

fn get_json_coordinates(reader: &mut impl JsonReader) -> Option<Coordinates> {
    let mut coordinates = Coordinates::default();
    let mut some = false;

    let position = reader.current_position(false);

    if let Some(data_pos) = position.data_pos {
        some = true;
        coordinates.index = Some(data_pos as usize);
    }

    if let Some(line_pos) = position.line_pos {
        some = true;
        coordinates.row_and_column = Some((line_pos.line as usize, Some(line_pos.column as usize)));
    };

    if some {
        Some(coordinates)
    } else {
        None
    }
}

use super::super::{super::normal::*, builder::*, *};

use {
    base64::{prelude::*, read::*},
    borc::{basic::streaming::*, errors::*},
    std::io::Read,
    tracing::trace,
};

//
// Reader
//

impl Reader {
    /// Reads from CBOR into a normal value.
    ///
    /// Is affected by [Reader::base64](super::super::Reader).
    pub fn read_cbor<ReadT>(&self, reader: &mut ReadT) -> Result<Value, ReadError>
    where
        ReadT: Read,
    {
        let mut value_builder = ValueBuilder::new();
        if self.base64 {
            let reader = DecoderReader::new(reader, &BASE64_STANDARD);
            let mut decoder = Decoder::new(reader);
            read_next_cbor(&mut decoder, &mut value_builder, None)?;
        } else {
            let mut decoder = Decoder::new(reader);
            read_next_cbor(&mut decoder, &mut value_builder, None)?;
        }
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_cbor<ReadT>(
    decoder: &mut Decoder<ReadT>,
    value_builder: &mut ValueBuilder,
    annotation: Option<Annotation>,
) -> Result<bool, ReadError>
where
    ReadT: Read,
{
    let event = decoder.next_event()?;
    trace!("{:?}", event);

    match event {
        Event::Break => {
            return Ok(false);
        }

        Event::Tag(tag) => {
            // https://www.rfc-editor.org/rfc/rfc8949.html#name-tagging-of-items
            return read_next_cbor(decoder, value_builder, Some(Annotation::Integer(tag as i64)));
        }

        Event::Null => {
            value_builder.add(Null::new().with_annotation(annotation));
        }

        Event::Unsigned(unsigned_integer) => {
            value_builder.add(UnsignedInteger::new(unsigned_integer).with_annotation(annotation));
        }

        Event::Signed(integer) => {
            match Event::interpret_signed_checked(integer) {
                Some(integer) => {
                    value_builder.add(Integer::new(integer).with_annotation(annotation));
                }

                None => return Err(DecodeError::Malformed.into()),
            };
        }

        Event::Float(float) => {
            value_builder.add(Float::new(float).with_annotation(annotation));
        }

        Event::Bool(boolean) => {
            value_builder.add(Boolean::new(boolean).with_annotation(annotation));
        }

        Event::TextString(string) => {
            value_builder.add(Text::new(string).with_annotation(annotation));
        }

        Event::UnknownLengthTextString => {
            let string = read_cbor_unknown_length_text_string(decoder)?;
            value_builder.add(Text::new(string).with_annotation(annotation));
        }

        Event::ByteString(bytes) => {
            value_builder.add(Bytes::new(bytes).with_annotation(annotation));
        }

        Event::UnknownLengthByteString => {
            let bytes = read_cbor_unknown_length_bytes(decoder)?;
            value_builder.add(Bytes::new(bytes).with_annotation(annotation));
        }

        Event::Array(length) => {
            value_builder.start_list_with_annotation(annotation);
            for _ in 0..length {
                read_next_cbor(decoder, value_builder, None)?;
            }
            value_builder.end_container();
        }

        Event::UnknownLengthArray => {
            value_builder.start_list_with_annotation(annotation);
            loop {
                match decoder.next_event()? {
                    Event::Break => {
                        break;
                    }

                    _ => {
                        read_next_cbor(decoder, value_builder, None)?;
                    }
                }
            }
            value_builder.end_container();
        }

        Event::Map(length) => {
            value_builder.start_map_with_annotation(annotation);
            for _ in 0..length {
                read_next_cbor(decoder, value_builder, None)?;
                read_next_cbor(decoder, value_builder, None)?;
            }
            value_builder.end_container();
        }

        Event::UnknownLengthMap => {
            value_builder.start_map_with_annotation(annotation);
            loop {
                match decoder.next_event()? {
                    Event::Break => {
                        break;
                    }

                    _ => {
                        read_next_cbor(decoder, value_builder, None)?;
                    }
                }
            }
            value_builder.end_container();
        }

        Event::Undefined => {}
    }

    Ok(true)
}

fn read_cbor_unknown_length_text_string<ReadT>(decoder: &mut Decoder<ReadT>) -> Result<String, DecodeError>
where
    ReadT: Read,
{
    let mut buffer = String::new();

    loop {
        match decoder.next_event()? {
            Event::TextString(string) => {
                buffer.push_str(&string);
            }

            Event::Break => return Ok(buffer),

            _ => return Err(DecodeError::Malformed),
        }
    }
}

fn read_cbor_unknown_length_bytes<ReadT>(decoder: &mut Decoder<ReadT>) -> Result<Vec<u8>, DecodeError>
where
    ReadT: Read,
{
    let mut buffer = Vec::new();

    loop {
        match decoder.next_event()? {
            Event::ByteString(bytes) => {
                buffer.extend_from_slice(&bytes);
            }

            Event::Break => return Ok(buffer),

            _ => return Err(DecodeError::Malformed),
        }
    }
}

use super::super::{
    super::{annotation::*, normal::*},
    builder::*,
    *,
};

use {
    base64::{prelude::*, read::*},
    borc::{basic::streaming::*, errors::*},
    std::io,
    tracing::trace,
};

impl Parser {
    /// Parses CBOR into a [Value].
    ///
    /// Is affected by [Parser::base64](super::super::Parser).
    pub fn parse_cbor<ReadT, AnnotationsT>(&self, reader: &mut ReadT) -> Result<Value<AnnotationsT>, ParseError>
    where
        ReadT: io::Read,
        AnnotationsT: Annotated + Clone + Default,
    {
        let mut value_builder = ValueBuilder::new(self.source.clone());
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

fn read_next_cbor<ReadT, AnnotationsT>(
    decoder: &mut Decoder<ReadT>,
    value_builder: &mut ValueBuilder<AnnotationsT>,
    label: Option<Label>,
) -> Result<bool, ParseError>
where
    ReadT: io::Read,
    AnnotationsT: Annotated + Clone + Default,
{
    let event = decoder.next_event()?;
    trace!("{:?}", event);

    match event {
        Event::Break => {
            return Ok(false);
        }

        Event::Tag(tag) => {
            // https://www.rfc-editor.org/rfc/rfc8949.html#name-tagging-of-items
            return read_next_cbor(decoder, value_builder, Some(Label::Integer(tag as i64)));
        }

        Event::Null => {
            value_builder.add(Null::default().with_label(label), None);
        }

        Event::Unsigned(unsigned_integer) => {
            value_builder.add(UnsignedInteger::new(unsigned_integer).with_label(label), None);
        }

        Event::Signed(integer) => {
            let integer = Event::interpret_signed_checked(integer).ok_or_else(|| DecodeError::Malformed)?;
            value_builder.add(Integer::new(integer).with_label(label), None);
        }

        Event::Float(float) => {
            value_builder.add(Float::from(float).with_label(label), None);
        }

        Event::Bool(boolean) => {
            value_builder.add(Boolean::new(boolean).with_label(label), None);
        }

        Event::TextString(string) => {
            value_builder.add(Text::from(string).with_label(label), None);
        }

        Event::UnknownLengthTextString => {
            let string = read_cbor_unknown_length_text_string(decoder)?;
            value_builder.add(Text::from(string).with_label(label), None);
        }

        Event::ByteString(bytes) => {
            value_builder.add(Blob::from(bytes).with_label(label), None);
        }

        Event::UnknownLengthByteString => {
            let bytes = read_cbor_unknown_length_bytes(decoder)?;
            value_builder.add(Blob::from(bytes).with_label(label), None);
        }

        Event::Array(length) => {
            value_builder.start_list_with_label(label, None);
            for _ in 0..length {
                read_next_cbor(decoder, value_builder, None)?;
            }
            value_builder.end_container();
        }

        Event::UnknownLengthArray => {
            value_builder.start_list_with_label(label, None);
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
            value_builder.start_map_with_label(label, None);
            for _ in 0..length {
                read_next_cbor(decoder, value_builder, None)?;
                read_next_cbor(decoder, value_builder, None)?;
            }
            value_builder.end_container();
        }

        Event::UnknownLengthMap => {
            value_builder.start_map_with_label(label, None);
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
    ReadT: io::Read,
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
    ReadT: io::Read,
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

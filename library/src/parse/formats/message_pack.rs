use super::super::{
    super::{
        annotate::*,
        normal::{Blob, *},
    },
    builder::*,
    *,
};

use {
    base64::{prelude::*, read::*},
    rmp::{decode::*, *},
    std::io,
    tracing::trace,
};

impl Parser {
    /// Parses MessagePack into a [Variant].
    ///
    /// Is affected by [Parser::base64](super::super::Parser).
    pub fn parse_message_pack<ReadT, AnnotatedT>(&self, reader: &mut ReadT) -> Result<Variant<AnnotatedT>, ParseError>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut value_builder = VariantBuilder::new(self.source.clone());
        if self.base64 {
            let mut reader = DecoderReader::new(reader, &BASE64_STANDARD);
            read_next_message_pack(&mut reader, &mut value_builder)?;
        } else {
            read_next_message_pack(reader, &mut value_builder)?;
        }
        Ok(value_builder.finalize())
    }
}

// Utils

fn read_next_message_pack<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    let marker = read_marker(reader)?;
    trace!("{:?}", marker);
    match marker {
        Marker::Reserved => {}

        Marker::Null => {
            value_builder.add(Null::default(), None);
        }

        Marker::True => {
            value_builder.add(Boolean::new(true), None);
        }

        Marker::False => {
            value_builder.add(Boolean::new(false), None);
        }

        Marker::FixNeg(integer) => {
            value_builder.add(Integer::new(integer as i64), None);
        }

        Marker::I8 => {
            value_builder.add(Integer::new(read_i8(reader)? as i64), None);
        }

        Marker::I16 => {
            value_builder.add(Integer::new(read_i16(reader)? as i64), None);
        }

        Marker::I32 => {
            value_builder.add(Integer::new(read_i32(reader)? as i64), None);
        }

        Marker::I64 => {
            value_builder.add(Integer::new(read_i64(reader)?), None);
        }

        Marker::FixPos(integer) => {
            value_builder.add(UnsignedInteger::new(integer as u64), None);
        }

        Marker::U8 => {
            value_builder.add(UnsignedInteger::new(read_u8(reader)? as u64), None);
        }

        Marker::U16 => {
            value_builder.add(UnsignedInteger::new(read_u16(reader)? as u64), None);
        }

        Marker::U32 => {
            value_builder.add(UnsignedInteger::new(read_u32(reader)? as u64), None);
        }

        Marker::U64 => {
            value_builder.add(UnsignedInteger::new(read_u64(reader)?), None);
        }

        Marker::F32 => {
            value_builder.add(Float::from(read_f32(reader)?), None);
        }

        Marker::F64 => {
            value_builder.add(Float::from(read_f64(reader)?), None);
        }

        Marker::Bin8 => {
            let length = read_u8(reader)? as usize;
            read_message_pack_bytes(reader, value_builder, length)?;
        }

        Marker::Bin16 => {
            let length = read_u16(reader)? as usize;
            read_message_pack_bytes(reader, value_builder, length)?;
        }

        Marker::Bin32 => {
            let length = read_u32(reader)? as usize;
            read_message_pack_bytes(reader, value_builder, length)?;
        }

        Marker::FixStr(length) => {
            read_message_pack_string(reader, value_builder, length as usize)?;
        }

        Marker::Str8 => {
            let length = read_u8(reader)? as usize;
            read_message_pack_string(reader, value_builder, length)?;
        }

        Marker::Str16 => {
            let length = read_u16(reader)? as usize;
            read_message_pack_string(reader, value_builder, length)?;
        }

        Marker::Str32 => {
            let length = read_u32(reader)? as usize;
            read_message_pack_string(reader, value_builder, length)?;
        }

        Marker::FixExt1 => {
            let label = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 1, label)?;
        }

        Marker::FixExt2 => {
            let label = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 2, label)?;
        }

        Marker::FixExt4 => {
            let label = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 4, label)?;
        }

        Marker::FixExt8 => {
            let label = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 8, label)?;
        }

        Marker::FixExt16 => {
            let label = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 16, label)?;
        }

        Marker::Ext8 => {
            let label = read_i8(reader)? as i64;
            let length = read_u8(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, label)?;
        }

        Marker::Ext16 => {
            let label = read_i8(reader)? as i64;
            let length = read_u16(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, label)?;
        }

        Marker::Ext32 => {
            let label = read_i8(reader)? as i64;
            let length = read_u32(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, label)?;
        }

        Marker::FixArray(length) => {
            read_message_pack_array(reader, value_builder, length as usize)?;
        }

        Marker::Array16 => {
            let length = read_u16(reader)? as usize;
            read_message_pack_array(reader, value_builder, length)?;
        }

        Marker::Array32 => {
            let length = read_u32(reader)? as usize;
            read_message_pack_array(reader, value_builder, length)?;
        }

        Marker::FixMap(length) => {
            read_message_pack_map(reader, value_builder, length as usize)?;
        }

        Marker::Map16 => {
            let length = read_u16(reader)? as usize;
            read_message_pack_map(reader, value_builder, length)?;
        }

        Marker::Map32 => {
            let length = read_u32(reader)? as usize;
            read_message_pack_map(reader, value_builder, length)?;
        }
    }

    Ok(())
}

fn read_message_pack_string<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
    length: usize,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    trace!("string length: {}", length);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    let string = String::from_utf8(buffer)?;
    Ok(value_builder.add(Text::from(string), None))
}

fn read_message_pack_bytes<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
    length: usize,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    trace!("bytes length: {}", length);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    Ok(value_builder.add(Blob::from(buffer), None))
}

fn read_message_pack_ext<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
    length: usize,
    label: i64,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    trace!("ext type: {}", label);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    Ok(value_builder.add(Blob::from(buffer).with_label(Some(Label::Integer(label))), None))
}

fn read_message_pack_array<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
    length: usize,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    trace!("array length: {}", length);
    value_builder.start_list(None);
    for _ in 0..length {
        read_next_message_pack(reader, value_builder)?;
    }
    value_builder.end_container();
    Ok(())
}

fn read_message_pack_map<ReadT, AnnotatedT>(
    reader: &mut ReadT,
    value_builder: &mut VariantBuilder<AnnotatedT>,
    length: usize,
) -> Result<(), ParseError>
where
    ReadT: io::Read,
    AnnotatedT: Annotated + Clone + Default,
{
    trace!("map length: {}", length);
    value_builder.start_map(None);
    for _ in 0..length {
        read_next_message_pack(reader, value_builder)?;
        read_next_message_pack(reader, value_builder)?;
    }
    value_builder.end_container();
    Ok(())
}

impl From<MarkerReadError> for ParseError {
    fn from(marker_read_error: MarkerReadError) -> Self {
        marker_read_error.0.into()
    }
}

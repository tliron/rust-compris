use super::super::{super::*, value_builder::*, *};

use {
    base64::{prelude::*, read::*},
    rmp::{decode::*, *},
    std::{io::Read, string::String as StdString},
    tracing::trace,
};

impl<R: Read> Reader<R> {
    /// Reads from MessagePack into a normal value.
    ///
    /// Is affected by [Reader::base64].
    pub fn read_message_pack(&mut self) -> Result<Value, ReadError> {
        let mut value_builder = ValueBuilder::new();
        if self.base64 {
            let mut reader = DecoderReader::new(self.reader.by_ref(), &BASE64_STANDARD);
            read_next_message_pack(&mut reader, &mut value_builder)?;
        } else {
            read_next_message_pack(self.reader.by_ref(), &mut value_builder)?;
        }
        Ok(value_builder.value())
    }
}

// Utils

fn read_next_message_pack<R: Read>(reader: &mut R, value_builder: &mut ValueBuilder) -> Result<(), ReadError> {
    let marker = read_marker(reader)?;
    trace!("{:?}", marker);
    match marker {
        Marker::Reserved => {
            trace!("reserved");
        }

        Marker::Null => {
            value_builder.add(Null::new());
        }

        Marker::True => {
            value_builder.add(Boolean::new(true));
        }

        Marker::False => {
            value_builder.add(Boolean::new(false));
        }

        Marker::FixNeg(integer) => {
            value_builder.add(Integer::new(integer as i64));
        }

        Marker::I8 => {
            value_builder.add(Integer::new(read_i8(reader)? as i64));
        }

        Marker::I16 => {
            value_builder.add(Integer::new(read_i16(reader)? as i64));
        }

        Marker::I32 => {
            value_builder.add(Integer::new(read_i32(reader)? as i64));
        }

        Marker::I64 => {
            value_builder.add(Integer::new(read_i64(reader)?));
        }

        Marker::FixPos(integer) => {
            value_builder.add(UnsignedInteger::new(integer as u64));
        }

        Marker::U8 => {
            value_builder.add(UnsignedInteger::new(read_u8(reader)? as u64));
        }

        Marker::U16 => {
            value_builder.add(UnsignedInteger::new(read_u16(reader)? as u64));
        }

        Marker::U32 => {
            value_builder.add(UnsignedInteger::new(read_u32(reader)? as u64));
        }

        Marker::U64 => {
            value_builder.add(UnsignedInteger::new(read_u64(reader)?));
        }

        Marker::F32 => {
            value_builder.add(Float::new(read_f32(reader)? as f64));
        }

        Marker::F64 => {
            value_builder.add(Float::new(read_f64(reader)?));
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
            let annotation = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 1, annotation)?;
        }

        Marker::FixExt2 => {
            let annotation = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 2, annotation)?;
        }

        Marker::FixExt4 => {
            let annotation = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 4, annotation)?;
        }

        Marker::FixExt8 => {
            let annotation = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 8, annotation)?;
        }

        Marker::FixExt16 => {
            let annotation = read_i8(reader)? as i64;
            read_message_pack_ext(reader, value_builder, 16, annotation)?;
        }

        Marker::Ext8 => {
            let annotation = read_i8(reader)? as i64;
            let length = read_u8(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, annotation)?;
        }

        Marker::Ext16 => {
            let annotation = read_i8(reader)? as i64;
            let length = read_u16(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, annotation)?;
        }

        Marker::Ext32 => {
            let annotation = read_i8(reader)? as i64;
            let length = read_u32(reader)? as usize;
            read_message_pack_ext(reader, value_builder, length, annotation)?;
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

fn read_message_pack_string<R: Read>(
    reader: &mut R,
    value_builder: &mut ValueBuilder,
    length: usize,
) -> Result<(), ReadError> {
    trace!("string length: {}", length);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    let string = StdString::from_utf8(buffer)?;
    Ok(value_builder.add(String::new(string)))
}

fn read_message_pack_bytes<R: Read>(
    reader: &mut R,
    value_builder: &mut ValueBuilder,
    length: usize,
) -> Result<(), ReadError> {
    trace!("bytes length: {}", length);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    Ok(value_builder.add(super::super::super::Bytes::new(buffer)))
}

fn read_message_pack_ext<R: Read>(
    reader: &mut R,
    value_builder: &mut ValueBuilder,
    length: usize,
    annotation: i64,
) -> Result<(), ReadError> {
    trace!("ext type: {}", annotation);
    let mut buffer = vec![0; length];
    reader.read_exact_buf(&mut buffer)?;
    Ok(value_builder.add(super::super::super::Bytes::new(buffer).with_annotation_integer(annotation)))
}

fn read_message_pack_array<R: Read>(
    reader: &mut R,
    value_builder: &mut ValueBuilder,
    length: usize,
) -> Result<(), ReadError> {
    trace!("array length: {}", length);
    value_builder.start_list();
    for _ in 0..length {
        read_next_message_pack(reader, value_builder)?;
    }
    value_builder.end_container();
    Ok(())
}

fn read_message_pack_map<R: Read>(
    reader: &mut R,
    value_builder: &mut ValueBuilder,
    length: usize,
) -> Result<(), ReadError> {
    trace!("map length: {}", length);
    value_builder.start_map();
    for _ in 0..length {
        read_next_message_pack(reader, value_builder)?;
        read_next_message_pack(reader, value_builder)?;
    }
    value_builder.end_container();
    Ok(())
}

impl From<MarkerReadError> for ReadError {
    fn from(value: MarkerReadError) -> Self {
        value.0.into()
    }
}

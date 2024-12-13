use super::super::*;

use {
    borc::{basic::streaming::*, errors::*},
    serde::{ser, Serialize},
    std::io::Write,
    tracing::trace,
};

impl Serializer {
    /// Serializes the provided value to the writer as CBOR.
    ///
    /// Is affected by [Serializer::base64](super::super::Serializer::base64).
    pub fn write_cbor<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: Write,
        SerializableT: Serialize + ?Sized,
    {
        fn write<W: Write, V: Serialize + ?Sized>(value: &V, writer: &mut W) -> Result<(), SerializeError> {
            Ok(value.serialize(&mut CborSerializer::new(writer))?)
        }

        if self.base64 {
            write(value, &mut Self::base64_writer(writer))?;
        } else {
            write(value, writer)?;
        }

        if self.pretty {
            Self::write_newline(writer)
        } else {
            Ok(())
        }
    }
}

//
// CborSerializer
//

struct CborSerializer<WriteT>
where
    WriteT: Write,
{
    encoder: Encoder<WriteT>,
}

impl<WriteT> CborSerializer<WriteT>
where
    WriteT: Write,
{
    fn new(writer: WriteT) -> Self {
        Self { encoder: Encoder::new(writer) }
    }

    fn event(self: &mut Self, event: Event) -> Result<(), EncodeError> {
        trace!("{:?}", event);
        self.encoder.feed_event(event)
    }
}

impl<'own, WriteT> ser::Serializer for &'own mut CborSerializer<WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;
    type SerializeSeq = CborSeqSerializer<'own, WriteT>;
    type SerializeTuple = CborTupleSerializer<'own, WriteT>;
    type SerializeTupleStruct = CborTupleStructSerializer<'own, WriteT>;
    type SerializeTupleVariant = CborTupleVariantSerializer<'own, WriteT>;
    type SerializeMap = CborMapSerializer<'own, WriteT>;
    type SerializeStruct = CborStructSerializer<'own, WriteT>;
    type SerializeStructVariant = CborStructVariantSerializer<'own, WriteT>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Bool(v))?)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        Ok(self.event(Event::create_signed(v as i64))?)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        Ok(self.event(Event::create_signed(v as i64))?)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        Ok(self.event(Event::create_signed(v as i64))?)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        Ok(self.event(Event::create_signed(v))?)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v as u64))?)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v as u64))?)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v as u64))?)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v))?)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Float(v as f64))?)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Float(v))?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v as u64))?)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::TextString(v.into()))?)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::ByteString(v.into()))?)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Null)?)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Tag(variant_index as u64))?;
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<SerializableT>(
        self,
        _name: &'static str,
        value: &SerializableT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<SerializableT>(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &SerializableT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        self.event(Event::Tag(variant_index as u64))?;
        self.event(Event::Map(1))?;
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => {
                self.event(Event::Array(len as u64))?;
                Ok(CborSeqSerializer { serializer: self, known: true })
            }
            None => {
                self.event(Event::UnknownLengthArray)?;
                Ok(CborSeqSerializer { serializer: self, known: false })
            }
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.event(Event::Array(len as u64))?;
        Ok(CborTupleSerializer { serializer: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.event(Event::Array(len as u64))?;
        Ok(CborTupleStructSerializer { serializer: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.event(Event::Tag(variant_index as u64))?;
        self.event(Event::Map(1))?;
        variant.serialize(&mut *self)?;
        self.event(Event::Array(len as u64))?;
        Ok(CborTupleVariantSerializer { serializer: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(len) => {
                self.event(Event::Map(len as u64))?;
                Ok(CborMapSerializer { serializer: self, known: true })
            }
            None => {
                self.event(Event::UnknownLengthMap)?;
                Ok(CborMapSerializer { serializer: self, known: false })
            }
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.event(Event::Map(len as u64))?;
        Ok(CborStructSerializer { serializer: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.event(Event::Tag(variant_index as u64))?;
        self.event(Event::Map(1))?;
        variant.serialize(&mut *self)?;
        self.event(Event::Map(len as u64))?;
        Ok(CborStructVariantSerializer { serializer: self })
    }
}

//
// CborSeqSerializer
//

pub struct CborSeqSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
    known: bool,
}

impl<'own, WriteT> ser::SerializeSeq for CborSeqSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_element<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(if self.known { () } else { self.serializer.event(Event::Break)? })
    }
}

//
// CborTupleSerializer
//

pub struct CborTupleSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
}

impl<'own, WriteT> ser::SerializeTuple for CborTupleSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_element<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleStructSerializer
//

pub struct CborTupleStructSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
}

impl<'own, WriteT> ser::SerializeTupleStruct for CborTupleStructSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_field<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleVariantSerializer
//

pub struct CborTupleVariantSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
}

impl<'own, WriteT> ser::SerializeTupleVariant for CborTupleVariantSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_field<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborMapSerializer
//

pub struct CborMapSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
    known: bool,
}

impl<'own, WriteT> ser::SerializeMap for CborMapSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_key<SerializableT>(&mut self, key: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(if self.known { () } else { self.serializer.event(Event::Break)? })
    }
}

//
// CborStructSerializer
//

pub struct CborStructSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
}

impl<'own, WriteT> ser::SerializeStruct for CborStructSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_field<SerializableT>(&mut self, key: &'static str, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborStructVariantSerializer
//

pub struct CborStructVariantSerializer<'own, WriteT>
where
    WriteT: Write,
{
    serializer: &'own mut CborSerializer<WriteT>,
}

impl<'own, WriteT> ser::SerializeStructVariant for CborStructVariantSerializer<'own, WriteT>
where
    WriteT: Write,
{
    type Ok = ();
    type Error = CborWriteError;

    fn serialize_field<SerializableT>(&mut self, key: &'static str, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

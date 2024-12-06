use super::super::errors::*;

use {
    borc::{basic::streaming::*, errors::*},
    serde::*,
    std::io::Write,
    tracing::trace,
};

impl<W: Write> super::super::serializer::Serializer<W> {
    pub fn write_cbor<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), WriteError> {
        fn write<V: Serialize + ?Sized>(value: &V, writer: impl Write) -> Result<(), WriteError> {
            Ok(value.serialize(&mut CborSerializer::new(writer))?)
        }

        if self.base64 {
            write(value, self.base64_writer())?;
        } else {
            write(value, self.writer.by_ref())?;
        }

        if self.pretty {
            self.write_newline()
        } else {
            Ok(())
        }
    }
}

//
// CborSerializer
//

pub struct CborSerializer<W: Write> {
    encoder: Encoder<W>,
}

impl<W: Write> CborSerializer<W> {
    pub fn new(writer: W) -> Self {
        Self { encoder: Encoder::new(writer) }
    }

    fn event(self: &mut Self, event: Event) -> Result<(), EncodeError> {
        trace!("{:?}", event);
        self.encoder.feed_event(event)
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut CborSerializer<W> {
    type Ok = ();
    type Error = CborError;
    type SerializeSeq = CborSeqSerializer<'a, W>;
    type SerializeTuple = CborTupleSerializer<'a, W>;
    type SerializeTupleStruct = CborTupleStructSerializer<'a, W>;
    type SerializeTupleVariant = CborTupleVariantSerializer<'a, W>;
    type SerializeMap = CborMapSerializer<'a, W>;
    type SerializeStruct = CborStructSerializer<'a, W>;
    type SerializeStructVariant = CborStructVariantSerializer<'a, W>;

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

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
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

pub struct CborSeqSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
    known: bool,
}

impl<'a, W: Write> ser::SerializeSeq for CborSeqSerializer<'a, W> {
    type Ok = ();

    type Error = CborError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(if self.known { () } else { self.serializer.event(Event::Break)? })
    }
}

//
// CborTupleSerializer
//

pub struct CborTupleSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
}

impl<'a, W: Write> ser::SerializeTuple for CborTupleSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleStructSerializer
//

pub struct CborTupleStructSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
}

impl<'a, W: Write> ser::SerializeTupleStruct for CborTupleStructSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleVariantSerializer
//

pub struct CborTupleVariantSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
}

impl<'a, W: Write> ser::SerializeTupleVariant for CborTupleVariantSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborMapSerializer
//

pub struct CborMapSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
    known: bool,
}

impl<'a, W: Write> ser::SerializeMap for CborMapSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(if self.known { () } else { self.serializer.event(Event::Break)? })
    }
}

//
// CborStructSerializer
//

pub struct CborStructSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
}

impl<'a, W: Write> ser::SerializeStruct for CborStructSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborStructSerializer
//

pub struct CborStructVariantSerializer<'a, W: Write> {
    serializer: &'a mut CborSerializer<W>,
}

impl<'a, W: Write> ser::SerializeStructVariant for CborStructVariantSerializer<'a, W> {
    type Ok = ();
    type Error = CborError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

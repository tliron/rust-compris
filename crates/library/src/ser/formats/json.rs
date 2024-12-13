use super::super::*;

use {
    serde::Serialize,
    std::io,
    struson::{serde::*, writer::*},
    tracing::trace,
};

impl Serializer {
    /// Serializes the provided value to the writer as JSON.
    pub fn write_json<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: io::Write,
        SerializableT: Serialize + ?Sized,
    {
        let json_stream_writer = if self.pretty {
            struson::writer::JsonStreamWriter::new_custom(
                writer.by_ref(),
                struson::writer::WriterSettings { pretty_print: true, ..Default::default() },
            )
        } else {
            struson::writer::JsonStreamWriter::new(writer.by_ref())
        };

        let mut json_stream_writer = StyledJsonWriter::new(json_stream_writer);

        json_stream_writer.serialize_value(&value)?;
        json_stream_writer.finish_document()?;

        if self.pretty {
            Self::write_newline(writer)
        } else {
            Ok(())
        }
    }
}

//
// StyledJsonWriter
//

// TODO: does nothing

struct StyledJsonWriter<JsonWriterT: JsonWriter> {
    writer: JsonWriterT,
}

impl<JsonWriterT> StyledJsonWriter<JsonWriterT>
where
    JsonWriterT: JsonWriter,
{
    fn new(writer: JsonWriterT) -> Self {
        Self { writer }
    }
}

impl<JsonWriterT> JsonWriter for StyledJsonWriter<JsonWriterT>
where
    JsonWriterT: JsonWriter,
{
    type WriterResult = JsonWriterT::WriterResult;

    fn begin_object(&mut self) -> Result<(), io::Error> {
        trace!("begin_object");
        self.writer.begin_object()
    }

    fn end_object(&mut self) -> Result<(), io::Error> {
        trace!("end_object");
        self.writer.end_object()
    }

    fn begin_array(&mut self) -> Result<(), io::Error> {
        trace!("begin_array");
        self.writer.begin_array()
    }

    fn end_array(&mut self) -> Result<(), io::Error> {
        trace!("end_array");
        self.writer.end_array()
    }

    fn name(&mut self, name: &str) -> Result<(), io::Error> {
        trace!("name {}", name);
        self.writer.name(name)
    }

    fn null_value(&mut self) -> Result<(), io::Error> {
        trace!("null_value");
        self.writer.null_value()
    }

    fn bool_value(&mut self, value: bool) -> Result<(), io::Error> {
        trace!("bool_value {}", value);
        self.writer.bool_value(value)
    }

    fn string_value(&mut self, value: &str) -> Result<(), io::Error> {
        trace!("string_value {}", value);
        self.writer.string_value(value)
    }

    fn string_value_writer(&mut self) -> Result<impl StringValueWriter + '_, io::Error> {
        trace!("bool_value_writer");
        self.writer.string_value_writer()
    }

    fn number_value_from_string(&mut self, value: &str) -> Result<(), JsonNumberError> {
        trace!("number_value_from_string {}", value);
        self.writer.number_value_from_string(value)
    }

    fn number_value<N: FiniteNumber>(&mut self, value: N) -> Result<(), io::Error> {
        trace!("number_value");
        self.writer.number_value(value)
    }

    fn fp_number_value<N: FloatingPointNumber>(&mut self, value: N) -> Result<(), JsonNumberError> {
        trace!("fp_number_value");
        self.writer.fp_number_value(value)
    }

    fn serialize_value<S: Serialize>(&mut self, value: &S) -> Result<(), SerializerError> {
        trace!("serialize_value");
        value.serialize(&mut JsonWriterSerializer::new(self))
    }

    fn finish_document(self) -> Result<Self::WriterResult, io::Error> {
        trace!("finish_document");
        self.writer.finish_document()
    }
}

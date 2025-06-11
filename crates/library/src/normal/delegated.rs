use super::{super::meta::*, value::*};

use {
    kutil_cli::debug::*,
    std::{fmt, io},
};

impl HasMeta for Value {
    fn get_meta(&self) -> Option<&Meta> {
        match self {
            Self::Nothing => None,
            Self::Null(null) => null.get_meta(),
            Self::Integer(integer) => integer.get_meta(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_meta(),
            Self::Float(float) => float.get_meta(),
            Self::Boolean(boolean) => boolean.get_meta(),
            Self::Text(text) => text.get_meta(),
            Self::Blob(blob) => blob.get_meta(),
            Self::List(list) => list.get_meta(),
            Self::Map(map) => map.get_meta(),
        }
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        match self {
            Self::Nothing => None,
            Self::Null(null) => null.get_meta_mut(),
            Self::Integer(integer) => integer.get_meta_mut(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_meta_mut(),
            Self::Float(float) => float.get_meta_mut(),
            Self::Boolean(boolean) => boolean.get_meta_mut(),
            Self::Text(text) => text.get_meta_mut(),
            Self::Blob(blob) => blob.get_meta_mut(),
            Self::List(list) => list.get_meta_mut(),
            Self::Map(map) => map.get_meta_mut(),
        }
    }
}

impl Debuggable for Value {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Nothing => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "Nothing")
            }
            Self::Null(null) => null.to_located().write_debug_for(writer, context),
            Self::Integer(integer) => integer.to_located().write_debug_for(writer, context),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.to_located().write_debug_for(writer, context),
            Self::Float(float) => float.to_located().write_debug_for(writer, context),
            Self::Boolean(boolean) => boolean.to_located().write_debug_for(writer, context),
            Self::Text(text) => text.to_located().write_debug_for(writer, context),
            Self::Blob(blob) => blob.to_located().write_debug_for(writer, context),
            Self::List(list) => list.write_debug_for(writer, context),
            Self::Map(map) => map.write_debug_for(writer, context),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => fmt::Display::fmt("nothing", formatter),
            Self::Null(null) => fmt::Display::fmt(null, formatter),
            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),
            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),
            Self::Float(float) => fmt::Display::fmt(float, formatter),
            Self::Boolean(boolean) => fmt::Display::fmt(boolean, formatter),
            Self::Text(text) => fmt::Display::fmt(text, formatter),
            Self::Blob(blob) => fmt::Display::fmt(blob, formatter),
            Self::List(list) => fmt::Display::fmt(list, formatter),
            Self::Map(map) => fmt::Display::fmt(map, formatter),
        }
    }
}

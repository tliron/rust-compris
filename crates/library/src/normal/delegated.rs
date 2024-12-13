use super::{super::meta::*, value::*};

use {
    kutil_cli::debug::*,
    owo_colors::*,
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
            Self::Bytes(bytes) => bytes.get_meta(),
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
            Self::Bytes(bytes) => bytes.get_meta_mut(),
            Self::List(list) => list.get_meta_mut(),
            Self::Map(map) => map.get_meta_mut(),
        }
    }
}

impl Debuggable for Value {
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Nothing => write!(writer, "{}", "nothing".style(theme.bare)),
            Self::Null(null) => null.to_located().write_debug_representation(writer, prefix, theme),
            Self::Integer(integer) => integer.to_located().write_debug_representation(writer, prefix, theme),
            Self::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.to_located().write_debug_representation(writer, prefix, theme)
            }
            Self::Float(float) => float.to_located().write_debug_representation(writer, prefix, theme),
            Self::Boolean(boolean) => boolean.to_located().write_debug_representation(writer, prefix, theme),
            Self::Text(text) => text.to_located().write_debug_representation(writer, prefix, theme),
            Self::Bytes(bytes) => bytes.to_located().write_debug_representation(writer, prefix, theme),
            Self::List(list) => list.write_debug_representation(writer, prefix, theme),
            Self::Map(map) => map.write_debug_representation(writer, prefix, theme),
        }
    }
}

impl Value {
    /// Compact version of [Debuggable::write_debug_representation].
    pub fn write_compact_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        theme: &Theme,
    ) -> Result<(), io::Error>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Nothing => write!(writer, "{}", "nothing".style(theme.bare)),
            Self::Null(null) => null.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::Integer(integer) => integer.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::UnsignedInteger(unsigned_integer) => {
                unsigned_integer.write_debug_representation(writer, &DebugPrefix::default(), theme)
            }
            Self::Float(float) => float.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::Boolean(boolean) => boolean.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::Text(text) => text.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::Bytes(bytes) => bytes.write_debug_representation(writer, &DebugPrefix::default(), theme),
            Self::List(list) => list.write_compact_debug_representation(writer, theme),
            Self::Map(map) => map.write_compact_debug_representation(writer, theme),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(formatter, "nothing"),
            Self::Null(null) => null.fmt(formatter),
            Self::Integer(integer) => integer.fmt(formatter),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.fmt(formatter),
            Self::Float(float) => float.fmt(formatter),
            Self::Boolean(boolean) => boolean.fmt(formatter),
            Self::Text(text) => text.fmt(formatter),
            Self::Bytes(bytes) => bytes.fmt(formatter),
            Self::List(list) => list.fmt(formatter),
            Self::Map(map) => map.fmt(formatter),
        }
    }
}

use super::{super::annotate::*, variant::*};

use {
    kutil::cli::depict::*,
    std::{cmp::*, fmt, hash::*, io},
};

impl<AnnotatedT> Annotated for Variant<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Undefined => None,
            Self::Null(null) => null.get_annotations(),
            Self::Integer(integer) => integer.get_annotations(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_annotations(),
            Self::Float(float) => float.get_annotations(),
            Self::Boolean(boolean) => boolean.get_annotations(),
            Self::Text(text) => text.get_annotations(),
            Self::Blob(blob) => blob.get_annotations(),
            Self::List(list) => list.get_annotations(),
            Self::Map(map) => map.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Undefined => None,
            Self::Null(null) => null.get_annotations_mut(),
            Self::Integer(integer) => integer.get_annotations_mut(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.get_annotations_mut(),
            Self::Float(float) => float.get_annotations_mut(),
            Self::Boolean(boolean) => boolean.get_annotations_mut(),
            Self::Text(text) => text.get_annotations_mut(),
            Self::Blob(blob) => blob.get_annotations_mut(),
            Self::List(list) => list.get_annotations_mut(),
            Self::Map(map) => map.get_annotations_mut(),
        }
    }
}

impl<AnnotatedT> Depict for Variant<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Undefined => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "Undefined")
            }
            Self::Null(null) => null.depict(writer, context),
            Self::Integer(integer) => integer.depict(writer, context),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.depict(writer, context),
            Self::Float(float) => float.depict(writer, context),
            Self::Boolean(boolean) => boolean.depict(writer, context),
            Self::Text(text) => text.depict(writer, context),
            Self::Blob(blob) => blob.depict(writer, context),
            Self::List(list) => list.depict(writer, context),
            Self::Map(map) => map.depict(writer, context),
        }
    }
}

impl<AnnotatedT> PartialEq for Variant<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null(_), Self::Null(_)) => true,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer == other_integer,
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer == other_unsigned_integer
            }
            (Self::Float(float), Self::Float(other_float)) => float == other_float,
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean == other_boolean,
            (Self::Text(text), Self::Text(other_text)) => text == other_text,
            (Self::Blob(blob), Self::Blob(other_blob)) => blob == other_blob,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            _ => false,
        }
    }
}

impl<AnnotatedT> Eq for Variant<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Variant<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null(_), Self::Null(_)) => Some(Ordering::Equal),
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.partial_cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.partial_cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.partial_cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.partial_cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.partial_cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.partial_cmp(other_blob),
            (Self::List(list), Self::List(other_list)) => list.partial_cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.partial_cmp(other_map),
            _ => None,
        }
    }
}

impl<AnnotatedT> Ord for Variant<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null(_), Self::Null(_)) => Ordering::Equal,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.cmp(other_blob),
            (Self::List(list), Self::List(other_list)) => list.cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.cmp(other_map),

            (Self::Undefined, _) => Ordering::Less,

            (Self::Null(_), Self::Undefined) => Ordering::Greater,
            (Self::Null(_), _) => Ordering::Less,

            (Self::Integer(_), Self::Undefined | Self::Null(_)) => Ordering::Greater,
            (Self::Integer(_), _) => Ordering::Less,

            (Self::UnsignedInteger(_), Self::Undefined | Self::Null(_) | Self::Integer(_)) => Ordering::Greater,
            (Self::UnsignedInteger(_), _) => Ordering::Less,

            (Self::Float(_), Self::Undefined | Self::Null(_) | Self::Integer(_) | Self::UnsignedInteger(_)) => {
                Ordering::Greater
            }
            (Self::Float(_), _) => Ordering::Less,

            (
                Self::Boolean(_),
                Self::Undefined | Self::Null(_) | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_),
            ) => Ordering::Greater,
            (Self::Boolean(_), _) => Ordering::Less,

            (
                Self::Text(_),
                Self::Undefined
                | Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_),
            ) => Ordering::Greater,
            (Self::Text(_), _) => Ordering::Less,

            (
                Self::Blob(_),
                Self::Undefined
                | Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_),
            ) => Ordering::Greater,
            (Self::Blob(_), _) => Ordering::Less,

            (
                Self::List(_),
                Self::Undefined
                | Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_),
            ) => Ordering::Greater,
            (Self::List(_), _) => Ordering::Less,

            (Self::Map(_), _) => Ordering::Less,
        }
    }
}

impl<AnnotatedT> Hash for Variant<AnnotatedT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Undefined => {
                state.write_u8(1);
            }

            Self::Null(_) => {
                state.write_u8(2);
            }

            Self::Integer(integer) => {
                state.write_u8(3);
                integer.hash(state);
            }

            Self::UnsignedInteger(unsigned_integer) => {
                state.write_u8(4);
                unsigned_integer.hash(state);
            }

            Self::Float(float) => {
                state.write_u8(5);
                float.hash(state);
            }

            Self::Boolean(boolean) => {
                state.write_u8(6);
                boolean.hash(state);
            }

            Self::Text(text) => {
                state.write_u8(7);
                text.hash(state);
            }

            Self::Blob(blob) => {
                state.write_u8(8);
                blob.hash(state);
            }

            Self::List(list) => {
                state.write_u8(9);
                list.hash(state);
            }

            Self::Map(map) => {
                state.write_u8(10);
                map.hash(state);
            }
        }
    }
}

impl<AnnotatedT> fmt::Display for Variant<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => fmt::Display::fmt("nothing", formatter),
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

use super::{super::annotate::*, variant::*};

use {
    kutil::cli::debug::*,
    std::{cmp::*, fmt, hash::*, io},
};

impl<AnnotatedT> Annotated for Variant<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn has_annotations() -> bool {
        AnnotatedT::has_annotations()
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

    fn set_annotations(&mut self, annotations: Annotations) {
        match self {
            Self::Undefined => {}
            Self::Null(null) => null.set_annotations(annotations),
            Self::Integer(integer) => integer.set_annotations(annotations),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.set_annotations(annotations),
            Self::Float(float) => float.set_annotations(annotations),
            Self::Boolean(boolean) => boolean.set_annotations(annotations),
            Self::Text(text) => text.set_annotations(annotations),
            Self::Blob(blob) => blob.set_annotations(annotations),
            Self::List(list) => list.set_annotations(annotations),
            Self::Map(map) => map.set_annotations(annotations),
        }
    }
}

impl<AnnotatedT> Debuggable for Variant<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Undefined => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "Nothing")
            }
            Self::Null(null) => null.write_debug_for(writer, context),
            Self::Integer(integer) => integer.write_debug_for(writer, context),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.write_debug_for(writer, context),
            Self::Float(float) => float.write_debug_for(writer, context),
            Self::Boolean(boolean) => boolean.write_debug_for(writer, context),
            Self::Text(text) => text.write_debug_for(writer, context),
            Self::Blob(blob) => blob.write_debug_for(writer, context),
            Self::List(list) => list.write_debug_for(writer, context),
            Self::Map(map) => map.write_debug_for(writer, context),
        }
    }
}

impl<AnnotatedT> PartialEq for Variant<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Undefined => return matches!(other, Self::Undefined),

            Self::Null(_) => return matches!(other, Self::Null(_)),

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    return integer == other_integer;
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    return unsigned_integer == other_unsigned_integer;
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    return float == other_float;
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    return boolean == other_boolean;
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    return text == other_text;
                }
            }

            Self::Blob(blob) => {
                if let Self::Blob(other_blob) = other {
                    return blob == other_blob;
                }
            }

            Self::List(list) => {
                if let Self::List(other_list) = other {
                    return list == other_list;
                }
            }

            Self::Map(map) => {
                if let Self::Map(other_map) = other {
                    return map == other_map;
                }
            }
        }

        false
    }
}

impl<AnnotatedT> Eq for Variant<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Variant<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Undefined => {
                if matches!(other, Self::Undefined) {
                    return Some(Ordering::Equal);
                }
            }

            Self::Null(_) => {
                if matches!(other, Self::Null(_)) {
                    return Some(Ordering::Equal);
                }
            }

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    return integer.partial_cmp(other_integer);
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    return unsigned_integer.partial_cmp(other_unsigned_integer);
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    return float.partial_cmp(other_float);
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    return boolean.partial_cmp(other_boolean);
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    return text.partial_cmp(other_text);
                }
            }

            Self::Blob(blob) => {
                if let Self::Blob(other_blob) = other {
                    return blob.partial_cmp(other_blob);
                }
            }

            Self::List(list) => {
                if let Self::List(other_list) = other {
                    return list.partial_cmp(other_list);
                }
            }

            Self::Map(map) => {
                if let Self::Map(other_map) = other {
                    return map.partial_cmp(other_map);
                }
            }
        }

        None
    }
}

impl<AnnotatedT> Ord for Variant<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Undefined => todo!(),

            Self::Null(_) => match other {
                Self::Null(_) => Ordering::Equal,
                _ => Ordering::Less,
            },

            Self::Integer(integer) => match other {
                Self::Null(_) => Ordering::Greater,
                Self::Integer(other_integer) => integer.cmp(other_integer),
                _ => Ordering::Less,
            },

            Self::UnsignedInteger(unsigned_integer) => match other {
                Self::Null(_) | Self::Integer(_) => Ordering::Greater,
                Self::UnsignedInteger(other_unsigned_integer) => unsigned_integer.cmp(other_unsigned_integer),
                _ => Ordering::Less,
            },

            Self::Float(float) => match other {
                Self::Null(_) | Self::Integer(_) | Self::UnsignedInteger(_) => Ordering::Greater,
                Self::Float(other_float) => float.cmp(other_float),
                _ => Ordering::Less,
            },

            Self::Boolean(boolean) => match other {
                Self::Null(_) | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) => Ordering::Greater,
                Self::Boolean(other_boolean) => boolean.cmp(other_boolean),
                _ => Ordering::Less,
            },

            Self::Text(text) => match other {
                Self::Null(_) | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_) => {
                    Ordering::Greater
                }

                Self::Text(other_text) => text.cmp(other_text),

                _ => Ordering::Less,
            },

            Self::Blob(blob) => match other {
                Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_) => Ordering::Greater,

                Self::Blob(other_blob) => blob.cmp(other_blob),

                _ => Ordering::Less,
            },

            Self::List(nested_list) => match other {
                Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_) => Ordering::Greater,

                Self::List(other_nested_list) => nested_list.cmp(other_nested_list),

                _ => Ordering::Less,
            },

            Self::Map(nested_map) => match other {
                Self::Undefined
                | Self::Null(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::List(_) => Ordering::Greater,

                Self::Map(other_nested_map) => nested_map.cmp(other_nested_map),
            },
        }
    }
}

impl<AnnotatedT> Hash for Variant<AnnotatedT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Undefined => {}
            Self::Null(null) => null.hash(state),
            Self::Integer(integer) => integer.hash(state),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.hash(state),
            Self::Float(float) => float.hash(state),
            Self::Boolean(boolean) => boolean.hash(state),
            Self::Text(text) => text.hash(state),
            Self::Blob(blob) => blob.hash(state),
            Self::List(list) => list.hash(state),
            Self::Map(map) => map.hash(state),
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

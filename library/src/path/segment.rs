use {
    kutil_cli::debug::*,
    kutil_std::zerocopy::*,
    std::{fmt, io},
};

//
// PathSegment
//

/// Path segment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PathSegment<KeyT> {
    /// List index.
    ListIndex(usize),

    /// Map key.
    MapKey(KeyT),
}

impl<KeyT> Debuggable for PathSegment<KeyT>
where
    KeyT: fmt::Display,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::ListIndex(index) => {
                context.theme.write_delimiter(writer, "[")?;
                context.theme.write_number(writer, index)?;
                context.theme.write_delimiter(writer, "]")
            }

            Self::MapKey(key) => {
                let key = key.to_string();
                let key = if key.contains("\"") || key.contains(char::is_whitespace) {
                    format!("{:?}", key)
                } else {
                    format!("{}", key)
                };
                context.theme.write_string(writer, key)
            }
        }
    }
}

impl<KeyT> fmt::Display for PathSegment<KeyT>
where
    KeyT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListIndex(index) => write!(formatter, "[{}]", index),

            Self::MapKey(key) => {
                let key = key.to_string();
                if key.contains("\"") || key.contains(char::is_whitespace) {
                    // Debug will quote the string
                    fmt::Debug::fmt(&key, formatter)
                } else {
                    fmt::Display::fmt(&key, formatter)
                }
            }
        }
    }
}

impl<KeyT> PathSegment<KeyT>
where
    KeyT: fmt::Display,
{
    /// To string keys.
    pub fn to_string_keys(&self) -> PathSegment<ByteString> {
        match self {
            Self::ListIndex(index) => PathSegment::ListIndex(*index),
            Self::MapKey(key) => PathSegment::MapKey(key.to_string().into()),
        }
    }
}

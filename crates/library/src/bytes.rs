use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    base64::{prelude::*, *},
    owo_colors::OwoColorize,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Bytes
//

/// Normal bytes value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Bytes {
    /// Actual value.
    pub value: Vec<u8>,

    /// Metadata.
    pub meta: Meta,
}

impl Bytes {
    /// Constructor.
    pub fn new(value: Vec<u8>) -> Self {
        Self { value, ..Default::default() }
    }

    /// Constructor.
    pub fn new_from_base64(value: &str) -> Result<Self, DecodeError> {
        let bytes = BASE64_STANDARD.decode(value)?;
        Ok(Self::new(bytes))
    }

    /// To Base64.
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

impl Into<Value> for Bytes {
    fn into(self) -> Value {
        Value::Bytes(self)
    }
}

impl HasMeta for Bytes {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Bytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}b64", self.to_base64())
    }
}

impl<W: io::Write> WriteDebug<W> for Bytes {
    fn write_debug_representation(&self, writer: &mut W, indentation: usize, styles: &Styles) -> Result<(), io::Error> {
        write!(writer, "{} bytes", self.value.len().style(styles.plain))?;
        if let Some(location) = &self.meta.location {
            location.write_debug_representation(writer, indentation, styles)?;
        }
        Ok(())
    }
}

impl ToMapStringKey for Bytes {
    fn to_map_string_key(&self) -> String {
        BASE64_STANDARD.encode(&self.value)
    }
}

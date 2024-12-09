use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use {
    ordermap::*,
    std::{cmp::*, fmt, hash::*, io, string::String as StdString},
};

//
// Map
//

/// Normal map value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Map {
    // Why we chose OrderMap:
    //
    // 1. HashMap cannot be hashed
    // 2. BTreeMap can be hashed, but does sorting, which we do not want
    // 3. OrderMap can be hashed, and does not do sorting;
    //    Of course it does retain insertion order... which might be useful?
    /// Actual value.
    pub value: OrderMap<Value, Value>,

    /// Metadata.
    pub meta: Meta,
}

impl Map {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with(value: impl Into<OrderMap<Value, Value>>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }
}

impl Value {
    /// If this is a map, gets a reference to a value in the map
    pub fn get(&self, key: impl Into<Self>) -> Option<&Self> {
        let key: Self = key.into();
        match self {
            Self::Map(map) => map.value.get(&key),
            _ => None,
        }
    }

    /// If this is a map, gets a mutable reference to a value in the map
    pub fn get_mut(&mut self, key: impl Into<Self>) -> Option<&mut Self> {
        let key: Self = key.into();
        match self {
            Value::Map(map) => map.value.get_mut(&key),
            _ => None,
        }
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Normal for Map {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        let mut buffer = '{'.to_string();
        let entries: Vec<StdString> =
            self.value.iter().map(|(k, v)| k.to_map_string_key() + ":" + &v.to_map_string_key()).collect();
        buffer.push_str(&entries.join(","));
        buffer.push('}');
        buffer
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{{")?;

        let mut i = self.value.iter().peekable();
        while let Some((key, value)) = i.next() {
            match i.peek() {
                Some(_) => write!(formatter, "{}:{},", key, value)?,
                None => write!(formatter, "{}:{}", key, value)?,
            }
        }

        write!(formatter, "}}")?;

        match &self.meta.location {
            Some(location) => write!(formatter, " {}", location),
            None => Ok(()),
        }
    }
}

impl<W: io::Write> WriteDebug<W> for Map {
    fn write_debug_representation(
        &self,
        writer: &mut W,
        mut indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let indent = " ".repeat(indentation);
        indentation += 2;

        let mut first = true;
        for (key, value) in self.value.iter() {
            if first {
                write!(writer, "? ")?;
                first = false;
            } else {
                write!(writer, "\n{}? ", indent)?;
            }

            key.write_debug_representation(writer, indentation, styles)?;
            write!(writer, "\n{}: ", indent)?;
            value.write_debug_representation(writer, indentation, styles)?;
        }

        Ok(())
    }
}

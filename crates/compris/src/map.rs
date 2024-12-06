use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use {
    ordermap::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Map
//

/// ARD map value.
#[derive(Debug, Default, Clone, Eq)]
pub struct Map {
    // Why we chose OrderMap:
    //
    // 1. HashMap cannot be hashed
    // 2. BTreeMap can be hashed, but does sorting, which we do not want
    // 3. OrderMap can be hashed, and does not do sorting;
    //    Of course it does retain insertion order... which might be useful?
    pub value: OrderMap<Value, Value>,
    pub meta: Meta,
}

impl Map {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Into<Value> for Map {
    fn into(self) -> Value {
        Value::Map(self)
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

impl HasMeta for Map {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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

impl WriteDebug for Map {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
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

impl ToMapStringKey for Map {
    fn to_map_string_key(&self) -> String {
        let mut buffer = '{'.to_string();
        let entries: Vec<String> =
            self.value.iter().map(|(k, v)| k.to_map_string_key() + ":" + &v.to_map_string_key()).collect();
        buffer.push_str(&entries.join(","));
        buffer.push('}');
        buffer
    }
}

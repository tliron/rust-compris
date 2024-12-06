use super::{meta::*, styles::*, to_map_string_key::*, value::*, write_debug::*};

use std::{cmp::*, fmt, hash::*, io};

//
// List
//

/// ARD list value.
#[derive(Debug, Default, Clone, Eq)]
pub struct List {
    pub value: Vec<Value>,
    pub meta: Meta,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Into<Value> for List {
    fn into(self) -> Value {
        Value::List(self)
    }
}

impl HasMeta for List {
    fn get_meta(&self) -> &Meta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut Meta {
        &mut self.meta
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for List {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for List {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[")?;

        let mut i = self.value.iter().peekable();
        while let Some(element) = i.next() {
            match i.peek() {
                Some(_) => write!(formatter, "{},", element)?,
                None => write!(formatter, "{}", element)?,
            }
        }

        write!(formatter, "]")?;

        match &self.meta.location {
            Some(location) => write!(formatter, " {}", location),
            None => Ok(()),
        }
    }
}

impl WriteDebug for List {
    fn write_debug_representation(
        &self,
        writer: &mut dyn io::Write,
        mut indentation: usize,
        styles: &Styles,
    ) -> Result<(), io::Error> {
        let indent = " ".repeat(indentation);
        indentation += 2;

        let mut first = true;
        for element in self.value.iter() {
            if first {
                write!(writer, "- ")?;
                first = false;
            } else {
                write!(writer, "\n{}- ", indent)?;
            }

            element.write_debug_representation(writer, indentation, styles)?;
        }

        Ok(())
    }
}

impl ToMapStringKey for List {
    fn to_map_string_key(&self) -> String {
        let mut buffer = '['.to_string();
        let elements: Vec<String> = self.value.iter().map(|e| e.to_map_string_key()).collect();
        buffer.push_str(&elements.join(","));
        buffer.push(']');
        buffer
    }
}

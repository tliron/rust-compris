use super::{
    super::{normal::*, styles::*, write_debug::*},
    meta::*,
};

use std::{cmp::*, fmt, hash::*, io, string::String as StdString};

//
// List
//

/// Normal list value.
#[derive(Debug, Default, Clone, Eq)]
pub struct List {
    /// Actual value.
    pub value: Vec<Value>,

    /// Metadata.
    pub meta: Meta,
}

impl List {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with(value: impl Into<Vec<Value>>) -> Self {
        Self { value: value.into(), ..Default::default() }
    }

    /// Push a clone of the value only if the list doesn't contain it.
    pub fn push_unique_clone(&mut self, value: &Value) -> bool {
        if self.value.contains(value) {
            false
        } else {
            self.value.push(value.clone());
            true
        }
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

impl Normal for List {
    fn get_meta(&self) -> Option<&Meta> {
        Some(&self.meta)
    }

    fn get_meta_mut(&mut self) -> Option<&mut Meta> {
        Some(&mut self.meta)
    }

    fn to_map_string_key(&self) -> StdString {
        let mut buffer = '['.to_string();
        let elements: Vec<StdString> = self.value.iter().map(|e| e.to_map_string_key()).collect();
        buffer.push_str(&elements.join(","));
        buffer.push(']');
        buffer
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

impl<W: io::Write> WriteDebug<W> for List {
    fn write_debug_representation(
        &self,
        writer: &mut W,
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

use super::{
    super::{hints::*, meta::*, normal::*},
    errors::*,
};

use kutil_std::collections::*;

//
// ValueBuilder
//

/// Builds a [Value] recursively and sequentially.
///
/// This is a utility for format parsers.
#[derive(Debug)]
pub struct ValueBuilder {
    /// Stack.
    pub stack: Vec<StackEntry>,

    /// Key stack.
    pub key_stack: Vec<Option<Value>>,

    /// Value references.
    pub references: FastHashMap<usize, Value>,
}

impl ValueBuilder {
    /// Constructor.
    pub fn new() -> Self {
        Self { stack: Vec::new(), key_stack: Vec::new(), references: FastHashMap::default() }
    }

    /// Returns the final built value.
    ///
    /// If there is no value, returns [Value::Nothing].
    ///
    /// Panics if the builder is in an indeterminate state (e.g.
    /// [end_container](Self::end_container) wasn't called when necessary).
    ///
    /// After calling this method, the builder can be reused to build a new
    /// value.
    pub fn value(&mut self) -> Value {
        // To ensure reusability
        self.key_stack = Vec::new();
        self.references = FastHashMap::default();

        match self.stack.len() {
            0 => Value::Nothing,
            1 => self.stack.remove(0).value,
            length => panic!("ValueBuilder in indeterminate state, stack length: {}", length),
        }
    }

    /// Adds a value to the builder.
    ///
    /// If we're currently in a list, will add it as an item to the list.
    ///
    /// If we're currently in a map, will add it as either a map key or a map
    /// value. Keys are expected first, values are expected next. Only when
    /// both key and value are added is the pair inserted into the map.
    ///
    /// Otherwise, will set the value to be the builder's final value.
    pub fn add<ValueT>(&mut self, value: ValueT, reference: Option<usize>)
    where
        ValueT: Into<Value>,
    {
        let value = value.into();

        if let Some(reference) = reference {
            tracing::trace!("add reference {} {}", reference, value);
            self.references.insert(reference, value.clone());
        }

        match self.stack.last_mut() {
            Some(entry) => match &mut entry.value {
                Value::List(list) => {
                    tracing::trace!("add to list: {}", value);
                    list.value.push(value);
                }

                Value::Map(map) => {
                    let key = self.key_stack.last_mut().expect("no key stack for map");
                    match key.take() {
                        None => {
                            // We don't have a key, so that means this is the key
                            tracing::trace!("set map key: {}", value);
                            *key = Some(value);
                        }

                        Some(key) => {
                            // We have the key, so that means this is the value
                            tracing::trace!("insert in map: {} -> {}", key, value);
                            map.value.insert(key, value);
                        }
                    }
                }

                _ => panic!("malformed: not a container: {}", entry.value),
            },

            None => {
                tracing::trace!("no container");
                self.push(value, reference);
            }
        }
    }

    /// Add a referenced [Value].
    pub fn add_referenced(&mut self, reference: usize) -> Result<(), ParseError> {
        match self.references.get(&reference) {
            Some(value) => {
                self.add(value.clone(), None);
                Ok(())
            }

            None => Err(ParseError::ReferenceNotFound(reference)),
        }
    }

    /// Starts building a [List].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list(&mut self, reference: Option<usize>) {
        self.push(List::new().into(), reference);
    }

    /// Starts building a [List] with [Meta].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_meta(&mut self, meta: Meta, reference: Option<usize>) {
        self.push(List::new().with_meta(meta).into(), reference);
    }

    /// Starts building a [List] with optional [Location].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_location(&mut self, location: Option<Location>, reference: Option<usize>) {
        self.start_list_with_meta(Meta::new().with_location(location), reference);
    }

    /// Starts building a [List] with an optional [Annotation].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_annotation(&mut self, annotation: Option<Annotation>, reference: Option<usize>) {
        self.start_list_with_meta(Meta::new().with_annotation(annotation), reference);
    }

    /// Starts building a [Map].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map(&mut self, reference: Option<usize>) {
        self.push(Map::new().into(), reference);

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a [Map] with [Meta].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_meta(&mut self, meta: Meta, reference: Option<usize>) {
        self.push(Map::new().with_meta(meta).into(), reference);

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a [Map] with optional [Location].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_location(&mut self, location: Option<Location>, reference: Option<usize>) {
        self.start_map_with_meta(Meta::new().with_location(location), reference);
    }

    /// Starts building a map [Map] an optional [Annotation].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_annotation(&mut self, annotation: Option<Annotation>, reference: Option<usize>) {
        self.start_map_with_meta(Meta::new().with_annotation(annotation), reference);
    }

    /// Ends building a container.
    ///
    /// Follows either [start_list](Self::start_list) or [start_map](Self::start_map).
    pub fn end_container(&mut self) {
        self.end_container_with_hints(None).expect("no hints means no parse errors")
    }

    /// Ends building a container with optional support for hint processing.
    ///
    /// See [Value::to_hinted_value].
    pub fn end_container_with_hints(&mut self, hints: Option<&Hints>) -> Result<(), ParseError> {
        let mut entry = self.stack.pop().expect("malformed: empty stack");

        tracing::trace!("pop from stack: {}", entry.value);

        if matches!(entry.value, Value::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        if let Some(hints) = hints {
            if let Some(hinted_value) = entry.value.to_hinted_value(hints)? {
                entry.value = hinted_value;
            }
        }

        self.add(entry.value, entry.reference);

        Ok(())
    }

    fn push(&mut self, value: Value, reference: Option<usize>) {
        tracing::trace!("push on stack: {}", value);
        self.stack.push(StackEntry::new(value, reference));
    }
}

//
// StackEntry
//

/// [ValueBuilder] stack entry.
#[derive(Debug)]
pub struct StackEntry {
    /// Value.
    pub value: Value,

    /// Optional reference.
    pub reference: Option<usize>,
}

impl StackEntry {
    /// Constructor.
    pub fn new(value: Value, reference: Option<usize>) -> Self {
        Self { value, reference }
    }
}

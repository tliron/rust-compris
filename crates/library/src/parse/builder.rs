use super::{
    super::{hints::*, meta::*, normal::*},
    errors::*,
};

use tracing::trace;

//
// ValueBuilder
//

/// Builds a [Value] recursively and sequentially.
///
/// This is a utility for format parsers.
pub struct ValueBuilder {
    stack: Vec<Value>,
    key_stack: Vec<Option<Value>>,
}

impl ValueBuilder {
    /// Constructor.
    pub fn new() -> Self {
        Self { stack: Vec::new(), key_stack: Vec::new() }
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
        match self.stack.len() {
            0 => Value::Nothing,
            1 => {
                let value = self.stack.remove(0);
                self.key_stack = Vec::new(); // to ensure reusability
                value
            }
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
    pub fn add<ValueT>(&mut self, value: ValueT)
    where
        ValueT: Into<Value>,
    {
        let value = value.into();
        match self.stack.last_mut() {
            Some(ref mut container) => match container {
                Value::List(list) => {
                    trace!("add to list: {}", value);
                    list.value.push(value)
                }

                Value::Map(map) => {
                    let key = self.key_stack.last_mut().expect("no key stack for map");
                    match key.take() {
                        None => {
                            // We don't have a key, so that means this is the key
                            trace!("set map key: {}", value);
                            *key = Some(value);
                        }

                        Some(key) => {
                            // We have the key, so that means this is the value
                            trace!("insert in map: {} -> {}", key, value);
                            map.value.insert(key, value);
                        }
                    }
                }

                _ => panic!("malformed: not a container: {}", container),
            },

            None => {
                trace!("no container");
                self.push(value);
            }
        }
    }

    /// Starts building a [List].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list(&mut self) {
        self.push(List::new());
    }

    /// Starts building a [List] with [Meta].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_meta(&mut self, meta: Meta) {
        self.push(List::new().with_meta(meta));
    }

    /// Starts building a [List] with optional [Location].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_location(&mut self, location: Option<Location>) {
        self.start_list_with_meta(Meta::new().with_location(location));
    }

    /// Starts building a [List] with an optional [Annotation].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_annotation(&mut self, annotation: Option<Annotation>) {
        self.start_list_with_meta(Meta::new().with_annotation(annotation));
    }

    /// Starts building a [Map].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map(&mut self) {
        self.push(Map::new());

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a [Map] with [Meta].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_meta(&mut self, meta: Meta) {
        self.push(Map::new().with_meta(meta));

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a [Map] with optional [Location].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_location(&mut self, location: Option<Location>) {
        self.start_map_with_meta(Meta::new().with_location(location));
    }

    /// Starts building a map [Map] an optional [Annotation].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_annotation(&mut self, annotation: Option<Annotation>) {
        self.start_map_with_meta(Meta::new().with_annotation(annotation));
    }

    /// Ends building a container.
    ///
    /// Follows either [start_list](Self::start_list) or [start_map](Self::start_map).
    pub fn end_container(&mut self) {
        _ = self.end_container_with_hints(None);
    }

    /// Ends building a container with optional support for hint processing.
    ///
    /// See [Value::to_hinted_value].
    pub fn end_container_with_hints(&mut self, hints: Option<&Hints>) -> Result<(), ParseError> {
        let mut value = self.stack.pop().expect("malformed: empty stack");

        trace!("pop from stack: {}", value);

        if matches!(value, Value::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        if let Some(hints) = hints {
            if let Some(hinted_value) = value.to_hinted_value(hints)? {
                value = hinted_value;
            }
        }

        self.add(value);

        Ok(())
    }

    fn push<ValueT>(&mut self, value: ValueT)
    where
        ValueT: Into<Value>,
    {
        let value = value.into();
        trace!("push on stack: {}", value);
        self.stack.push(value)
    }
}

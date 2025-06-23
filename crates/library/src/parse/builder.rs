use super::{
    super::{annotation::*, hints::*, normal::*},
    error::*,
};

use {bytestring::*, kutil_std::collections::*};

//
// ValueBuilder
//

/// Builds a [Value] sequentially.
///
/// This is a utility for format parsers.
#[derive(Debug)]
pub struct ValueBuilder<AnnotationsT> {
    /// Source.
    pub source: Option<ByteString>,

    /// Stack.
    pub stack: Vec<StackEntry<AnnotationsT>>,

    /// Key stack.
    pub key_stack: Vec<Option<Value<AnnotationsT>>>,

    /// Value references.
    pub references: FastHashMap<usize, Value<AnnotationsT>>,
}

impl<AnnotationsT> ValueBuilder<AnnotationsT> {
    /// Constructor.
    pub fn new(source: Option<ByteString>) -> Self {
        Self { source, stack: Vec::new(), key_stack: Vec::new(), references: FastHashMap::default() }
    }

    /// Returns the final built value.
    ///
    /// If there is no value, returns [Value::Nothing].
    ///
    /// Panics if the builder is in an indeterminate state (e.g.
    /// [end_container](Self::end_container) wasn't called when necessary).
    ///
    /// After calling this method the builder can be reused to build a new value.
    pub fn value(&mut self) -> Value<AnnotationsT>
    where
        AnnotationsT: Annotated + Default,
    {
        // To ensure reusability
        self.key_stack = Vec::new();
        self.references = FastHashMap::default();

        match self.stack.len() {
            0 => Value::Nothing,
            1 => self.stack.remove(0).value.annotated(&self.source),
            length => panic!("ValueBuilder in indeterminate state, stack length: {}", length),
        }
    }

    /// Adds a value to the builder.
    ///
    /// If we're currently in a list, will add it as an item to the list.
    ///
    /// If we're currently in a map, will add it as either a map key or a map value. Keys are
    /// expected first, values are expected next. Only when both key and value are added is the
    /// pair inserted into the map.
    ///
    /// Otherwise, will set the value to be the builder's final value.
    pub fn add<ValueT>(&mut self, value: ValueT, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Clone,
        ValueT: Into<Value<AnnotationsT>>,
    {
        let value = value.into();

        if let Some(reference) = reference {
            // Note that we are allowing overriding of references
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
    pub fn add_referenced(&mut self, reference: usize) -> Result<(), ParseError>
    where
        AnnotationsT: Annotated + Clone,
    {
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
    pub fn start_list(&mut self, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(List::default().into(), reference);
    }

    /// Starts building a [List] with optional [Span].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_span(&mut self, span: Option<Span>, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(List::default().with_span(span).into(), reference);
    }

    /// Starts building a [List] with an optional [Label].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_label(&mut self, label: Option<Label>, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(List::default().with_label(label).into(), reference);
    }

    /// Starts building a [Map].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map(&mut self, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(Map::default().into(), reference);

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a [Map] with optional [Span].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_span(&mut self, span: Option<Span>, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(Map::default().with_span(span).into(), reference);

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Starts building a map [Map] an optional [Label].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map_with_label(&mut self, label: Option<Label>, reference: Option<usize>)
    where
        AnnotationsT: Annotated + Default,
    {
        self.push(Map::default().with_label(label).into(), reference);

        // Every map entry on the stack has a matching key_stack entry
        self.key_stack.push(None);
    }

    /// Ends building a container.
    ///
    /// Follows either [start_list](Self::start_list) or [start_map](Self::start_map).
    pub fn end_container(&mut self)
    where
        AnnotationsT: Annotated + Clone,
    {
        let entry = self.stack.pop().expect("malformed: empty stack");

        tracing::trace!("pop from stack: {}", entry.value);

        if matches!(entry.value, Value::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        self.add(entry.value, entry.reference);
    }

    /// Ends building a container with optional support for hint processing.
    ///
    /// See [Value::to_hinted_value].
    pub fn end_container_with_hints(&mut self, hints: Option<&Hints>) -> Result<(), ParseError>
    where
        AnnotationsT: Annotated + Clone + Default,
    {
        let mut entry = self.stack.pop().expect("malformed: empty stack");

        tracing::trace!("pop from stack: {}", entry.value);

        if matches!(entry.value, Value::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        if let Some(hints) = hints
            && let Some(hinted_value) = entry.value.to_hinted_value(hints)?
        {
            entry.value = hinted_value;
        }

        self.add(entry.value, entry.reference);

        Ok(())
    }

    fn push(&mut self, value: Value<AnnotationsT>, reference: Option<usize>)
    where
        AnnotationsT: Annotated,
    {
        tracing::trace!("push on stack: {}", value);
        self.stack.push(StackEntry::new(value, reference));
    }
}

//
// StackEntry
//

/// [ValueBuilder] stack entry.
#[derive(Debug)]
pub struct StackEntry<AnnotationsT> {
    /// Value.
    pub value: Value<AnnotationsT>,

    /// Optional reference.
    pub reference: Option<usize>,
}

impl<AnnotationsT> StackEntry<AnnotationsT> {
    /// Constructor.
    pub fn new(value: Value<AnnotationsT>, reference: Option<usize>) -> Self {
        Self { value, reference }
    }
}

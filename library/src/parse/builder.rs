use super::{
    super::{annotate::*, hints::*, normal::*},
    error::*,
};

use kutil::std::{collections::*, immutable::*};

//
// VariantBuilder
//

/// Builds a [Variant] sequentially.
///
/// This is a utility for format parsers.
#[derive(Debug)]
pub struct VariantBuilder<AnnotatedT> {
    /// Source.
    pub source: Option<ByteString>,

    /// Stack.
    pub stack: Vec<StackEntry<AnnotatedT>>,

    /// Key stack.
    pub key_stack: Vec<Option<Variant<AnnotatedT>>>,

    /// Variant references.
    pub references: FastHashMap<usize, Variant<AnnotatedT>>,
}

impl<AnnotatedT> VariantBuilder<AnnotatedT> {
    /// Constructor.
    pub fn new(source: Option<ByteString>) -> Self {
        Self { source, stack: Default::default(), key_stack: Default::default(), references: FastHashMap::default() }
    }

    /// Returns the final built [Variant].
    ///
    /// If there is no variant, returns [Variant::Undefined].
    ///
    /// Panics if the builder is in an indeterminate state (e.g.
    /// [end_container](Self::end_container) wasn't called when necessary).
    ///
    /// After calling this method the builder can be reused to build a new variant.
    pub fn finalize(&mut self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Default,
    {
        // To ensure reusability
        self.key_stack = Default::default();
        self.references = FastHashMap::default();

        match self.stack.len() {
            0 => Variant::Undefined,
            1 => self.stack.remove(0).variant.fully_annotated(&self.source),
            length => panic!("VariantBuilder in indeterminate state, stack length: {}", length),
        }
    }

    /// Adds a variant to the builder.
    ///
    /// If we're currently in a list, will add it as an item to the list.
    ///
    /// If we're currently in a map, will add it as either a map key or a map value. Keys are
    /// expected first, values are expected next. Only when both key and value are added is the
    /// pair inserted into the map.
    ///
    /// Otherwise, will set the variant to be the builder's final variant.
    pub fn add<VariantT>(&mut self, variant: VariantT, reference: Option<usize>)
    where
        AnnotatedT: Annotated + Clone,
        VariantT: Into<Variant<AnnotatedT>>,
    {
        let variant = variant.into();

        if let Some(reference) = reference {
            // Note that we are allowing overriding of references
            tracing::trace!("add reference {} {}", reference, variant);
            self.references.insert(reference, variant.clone());
        }

        match self.stack.last_mut() {
            Some(entry) => match &mut entry.variant {
                Variant::List(list) => {
                    tracing::trace!("add to list: {}", variant);
                    list.inner.push(variant);
                }

                Variant::Map(map) => {
                    let key = self.key_stack.last_mut().expect("no key stack for map");
                    match key.take() {
                        None => {
                            // We don't have a key, so that means this is the key
                            tracing::trace!("set map key: {}", variant);
                            *key = Some(variant);
                        }

                        Some(key) => {
                            // We have the key, so that means this is the value
                            tracing::trace!("insert in map: {} -> {}", key, variant);
                            map.inner.insert(key, variant);
                        }
                    }
                }

                _ => panic!("malformed: not a container: {}", entry.variant),
            },

            None => {
                tracing::trace!("no container");
                self.push(variant, reference);
            }
        }
    }

    /// Add a referenced [Variant].
    pub fn add_referenced(&mut self, reference: usize) -> Result<(), ParseError>
    where
        AnnotatedT: Annotated + Clone,
    {
        match self.references.get(&reference) {
            Some(variant) => {
                self.add(variant.clone(), None);
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
        AnnotatedT: Annotated + Default,
    {
        self.push(List::default().into(), reference);
    }

    /// Starts building a [List] with optional [Span].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_span(&mut self, span: Option<Span>, reference: Option<usize>)
    where
        AnnotatedT: Annotated + Default,
    {
        self.push(List::default().with_span(span).into(), reference);
    }

    /// Starts building a [List] with an optional [Label].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_list_with_label(&mut self, label: Option<Label>, reference: Option<usize>)
    where
        AnnotatedT: Annotated + Default,
    {
        self.push(List::default().with_label(label).into(), reference);
    }

    /// Starts building a [Map].
    ///
    /// Should be followed later by [end_container](Self::end_container).
    pub fn start_map(&mut self, reference: Option<usize>)
    where
        AnnotatedT: Annotated + Default,
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
        AnnotatedT: Annotated + Default,
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
        AnnotatedT: Annotated + Default,
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
        AnnotatedT: Annotated + Clone,
    {
        let entry = self.stack.pop().expect("malformed: empty stack");

        tracing::trace!("pop from stack: {}", entry.variant);

        if matches!(entry.variant, Variant::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        self.add(entry.variant, entry.reference);
    }

    /// Ends building a container with optional support for hint processing.
    ///
    /// See [Variant::to_hinted_variant].
    pub fn end_container_with_hints(&mut self, hints: Option<&Hints>) -> Result<(), ParseError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut entry = self.stack.pop().expect("malformed: empty stack");

        tracing::trace!("pop from stack: {}", entry.variant);

        if matches!(entry.variant, Variant::Map(_)) {
            // Every map entry on the stack has a matching key_stack entry
            self.key_stack.pop();
        }

        if let Some(hints) = hints
            && let Some(hinted_value) = entry.variant.to_hinted_variant(hints)?
        {
            entry.variant = hinted_value;
        }

        self.add(entry.variant, entry.reference);

        Ok(())
    }

    fn push(&mut self, variant: Variant<AnnotatedT>, reference: Option<usize>)
    where
        AnnotatedT: Annotated,
    {
        tracing::trace!("push on stack: {}", variant);
        self.stack.push(StackEntry::new(variant, reference));
    }
}

//
// StackEntry
//

/// [VariantBuilder] stack entry.
#[derive(Debug)]
pub struct StackEntry<AnnotatedT> {
    /// Variant.
    pub variant: Variant<AnnotatedT>,

    /// Optional reference.
    pub reference: Option<usize>,
}

impl<AnnotatedT> StackEntry<AnnotatedT> {
    /// Constructor.
    pub fn new(variant: Variant<AnnotatedT>, reference: Option<usize>) -> Self {
        Self { variant, reference }
    }
}
